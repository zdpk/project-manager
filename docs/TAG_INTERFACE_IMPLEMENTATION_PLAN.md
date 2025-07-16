# Tag Interface Implementation Plan

## Current Implementation Analysis

### 현재 구현의 문제점

1. **단일 입력 방식**: `Text::new("🏷️  Tags:")` 사용으로 한 번에 모든 태그를 입력해야 함
2. **제한적인 상호작용**: 실시간 피드백 없음, 타이핑 중 미리보기 불가능
3. **복잡한 후처리**: 입력 후 별도의 확인 단계가 필요
4. **일관성 없는 UI**: 빈 입력과 태그 입력 시 완전히 다른 플로우

### 현재 코드 위치
- **파일**: `src/commands/project.rs`
- **함수**: `select_tags_interactive()` (line 223-338)
- **사용 라이브러리**: `inquire` crate의 `Text`와 `MultiSelect`

## Implementation Strategy

### Phase 1: Custom Interactive Tag Selector

`inquire` 라이브러리의 제한사항을 극복하기 위해 커스텀 인터랙티브 태그 선택기를 구현합니다.

#### 1.1 New Module Structure
```
src/
├── commands/
│   └── project.rs           # 기존 코드 수정
├── ui/                      # 새 모듈
│   ├── mod.rs
│   ├── tag_selector.rs      # 메인 태그 선택 UI
│   ├── input_handler.rs     # 키보드 입력 처리
│   └── display.rs           # 화면 렌더링
```

#### 1.2 Core Components

##### TagSelector State Machine
```rust
pub struct TagSelector {
    state: TagInputState,
    existing_tags: Vec<(String, usize)>, // (tag, usage_count)
    input_buffer: String,
    selected_tags: Vec<String>,
    cursor_position: usize,
    matches: Vec<String>,
}

pub enum TagInputState {
    Empty,                    // 초기 상태
    Typing,                   // 타이핑 중
    Browsing,                 // 기존 태그 브라우징
    Confirming,              // 선택 확인
}
```

##### Input Handler
```rust
pub enum KeyAction {
    Continue,
    Complete(Vec<String>),
    Cancel,
}

impl TagSelector {
    pub fn handle_key(&mut self, key: KeyEvent) -> KeyAction {
        match key.code {
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Char(' ') => self.handle_space(),
            KeyCode::Char(c) => self.handle_char(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Up => self.handle_up(),
            KeyCode::Down => self.handle_down(),
            KeyCode::Esc => KeyAction::Cancel,
            _ => KeyAction::Continue,
        }
    }
}
```

### Phase 2: Real-time UI Rendering

#### 2.1 Display Logic
```rust
impl TagSelector {
    fn render(&self) -> String {
        match self.state {
            TagInputState::Empty => self.render_empty_state(),
            TagInputState::Typing => self.render_typing_state(),
            TagInputState::Browsing => self.render_browsing_state(),
            TagInputState::Confirming => self.render_confirming_state(),
        }
    }
    
    fn render_empty_state(&self) -> String {
        format!(
            "🏷️  Tags: {}\n{}",
            self.input_buffer,
            "(Enter to create project without tags)"
        )
    }
    
    fn render_typing_state(&self) -> String {
        let matches_info = if self.matches.is_empty() {
            format!("(Enter to create tag `{}`)", self.input_buffer)
        } else {
            format!("(Enter to create project with selected tags)")
        };
        
        let mut output = format!("🏷️  Tags: {}\n{}\n", self.input_buffer, matches_info);
        
        // Show selected tags
        for tag in &self.selected_tags {
            output.push_str(&format!("[✓] {}\n", tag));
        }
        
        // Show matching tags
        for (i, tag) in self.matches.iter().enumerate() {
            let selected = if i == self.cursor_position { ">" } else { " " };
            let checked = if self.selected_tags.contains(tag) { "✓" } else { " " };
            output.push_str(&format!("{}[{}] {}\n", selected, checked, tag));
        }
        
        output
    }
}
```

#### 2.2 Terminal Control
```rust
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

impl TagSelector {
    pub async fn run(&mut self) -> Result<Vec<String>> {
        terminal::enable_raw_mode()?;
        
        loop {
            self.clear_screen()?;
            self.display_current_state()?;
            
            if let Event::Key(key_event) = event::read()? {
                match self.handle_key(key_event) {
                    KeyAction::Continue => continue,
                    KeyAction::Complete(tags) => {
                        terminal::disable_raw_mode()?;
                        return Ok(tags);
                    },
                    KeyAction::Cancel => {
                        terminal::disable_raw_mode()?;
                        return Ok(vec![]);
                    },
                }
            }
        }
    }
}
```

### Phase 3: Fuzzy Matching Enhancement

#### 3.1 Advanced Matching Algorithm
```rust
pub struct TagMatcher {
    tags: Vec<(String, usize)>,
}

impl TagMatcher {
    pub fn find_matches(&self, query: &str) -> Vec<MatchResult> {
        let mut matches = Vec::new();
        
        for (tag, count) in &self.tags {
            if let Some(score) = self.calculate_match_score(tag, query) {
                matches.push(MatchResult {
                    tag: tag.clone(),
                    count: *count,
                    score,
                    match_type: self.determine_match_type(tag, query),
                });
            }
        }
        
        // Sort by score (exact matches first, then by usage count)
        matches.sort_by(|a, b| {
            a.score.partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .reverse()
                .then_with(|| b.count.cmp(&a.count))
        });
        
        matches
    }
    
    fn calculate_match_score(&self, tag: &str, query: &str) -> Option<f32> {
        let tag_lower = tag.to_lowercase();
        let query_lower = query.to_lowercase();
        
        if tag_lower == query_lower {
            Some(1.0) // Exact match
        } else if tag_lower.starts_with(&query_lower) {
            Some(0.8) // Prefix match
        } else if tag_lower.contains(&query_lower) {
            Some(0.6) // Contains match
        } else {
            // Fuzzy match using Levenshtein distance
            let distance = levenshtein_distance(&tag_lower, &query_lower);
            let max_len = tag.len().max(query.len());
            let similarity = 1.0 - (distance as f32 / max_len as f32);
            
            if similarity > 0.5 {
                Some(similarity * 0.4) // Fuzzy match
            } else {
                None
            }
        }
    }
}
```

### Phase 4: Integration and Testing

#### 4.1 Integration Points
1. **Replace existing function**: `select_tags_interactive()`를 새 구현으로 교체
2. **Maintain compatibility**: 기존 API 유지하여 다른 코드 변경 최소화
3. **Error handling**: 터미널 제어 실패 시 fallback to 기존 방식

#### 4.2 Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let matcher = TagMatcher::new(vec![
            ("rust".to_string(), 5),
            ("javascript".to_string(), 3),
        ]);
        
        let matches = matcher.find_matches("rust");
        assert_eq!(matches[0].tag, "rust");
        assert_eq!(matches[0].score, 1.0);
    }

    #[test]
    fn test_prefix_match() {
        let matcher = TagMatcher::new(vec![
            ("rust".to_string(), 5),
            ("ruby".to_string(), 3),
        ]);
        
        let matches = matcher.find_matches("ru");
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|m| m.tag == "rust"));
        assert!(matches.iter().any(|m| m.tag == "ruby"));
    }
}
```

## Implementation Roadmap

### Week 1: Core Infrastructure
- [ ] Create UI module structure
- [ ] Implement TagSelector state machine
- [ ] Basic terminal control setup
- [ ] Simple input handling

### Week 2: Interactive Features
- [ ] Real-time display rendering
- [ ] Keyboard navigation
- [ ] Tag selection/deselection
- [ ] State transitions

### Week 3: Advanced Matching
- [ ] Fuzzy matching algorithm
- [ ] Performance optimization
- [ ] Match scoring system
- [ ] Usage-based sorting

### Week 4: Integration & Polish
- [ ] Replace existing implementation
- [ ] Error handling and fallbacks
- [ ] Comprehensive testing
- [ ] Documentation updates

## Dependencies

### New Crate Dependencies
```toml
[dependencies]
crossterm = "0.27"              # Terminal control
levenshtein = "1.0"             # Fuzzy matching
unicode-width = "0.1"           # Proper text width calculation
```

### Compatibility Considerations
- **Fallback support**: 터미널이 raw mode를 지원하지 않는 경우 기존 방식 사용
- **Testing environments**: CI/CD에서도 동작할 수 있도록 headless 모드 지원
- **Cross-platform**: Windows, macOS, Linux 모두에서 동작 보장

이 계획을 통해 사용자가 요청한 직관적이고 효율적인 태그 선택 인터페이스를 구현할 수 있습니다.