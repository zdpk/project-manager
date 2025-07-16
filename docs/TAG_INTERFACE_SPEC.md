# Enhanced Tag Selection Interface Specification

## Current Issues

현재 태그 선택 인터페이스에서 발생하는 문제들:

1. **어색한 입력 표시**: `🏷️  Tags: ()` 형태로 괄호가 표시되어 혼란스러움
2. **입력 시 괄호 무시**: 사용자가 타이핑할 때 괄호 뒤에 텍스트가 붙어서 어색함
3. **불명확한 상호작용**: 사용자가 다음에 무엇을 해야 할지 명확하지 않음
4. **일관성 없는 UI 플로우**: 각 상황별로 다른 동작 방식

## Improved Tag Selection Interface Specification

### Design Goals

1. **직관적인 인터페이스**: 사용자가 즉시 이해할 수 있는 명확한 UI
2. **일관된 상호작용**: 모든 상황에서 동일한 키 바인딩과 동작
3. **실시간 피드백**: 타이핑하는 동안 즉시 결과 표시
4. **효율적인 워크플로우**: 최소한의 키 입력으로 원하는 결과 달성

### Interface Flow Specification

#### Case 1: 빈 입력 상태 (Initial State)
```
🏷️  Tags: 
(Enter to create project without tags)
```

**동작:**
- `Enter`: 태그 없이 프로젝트 생성
- `문자 입력`: Case 2 또는 Case 3으로 전환

#### Case 2: 새로운 태그 생성 (New Tag Creation)
```
🏷️  Tags: a
(Enter to create tag `a`)
```

**조건:** 입력한 텍스트와 정확히 일치하는 기존 태그가 없는 경우

**동작:**
- `Enter`: 새 태그 `a` 생성하고 Case 2-2로 전환
- `Backspace`: 문자 삭제, 빈 상태면 Case 1로 전환
- `Space`: 현재 입력 무시하고 브라우저 모드로 전환
- `문자 추가`: 계속 타이핑, 실시간으로 기존 태그와 매칭 확인

#### Case 2-2: 태그 생성 완료 후 추가 선택
```
🏷️  Tags: 
(Enter to create project with selected tags)
[✓] a

Available tags:
[ ] backend
[ ] frontend
[ ] rust
```

**동작:**
- `Enter`: 선택된 태그들로 프로젝트 생성
- `Space`: 첫 번째 가용 태그 선택/해제
- `↑/↓`: 태그 목록 탐색
- `Space (특정 태그에서)`: 해당 태그 선택/해제
- `문자 입력`: 새로운 태그 검색 시작

#### Case 3: 기존 태그 검색 및 선택 (Existing Tag Search)
```
🏷️  Tags: a
(Enter to create project with selected tags)
[ ] abc
[ ] abd  
[✓] api
[ ] xab
```

**조건:** 입력한 텍스트가 포함된 기존 태그들이 있는 경우

**동작:**
- `Enter`: 현재 선택된 태그들로 프로젝트 생성
- `Space`: 첫 번째 태그 선택/해제
- `↑/↓`: 태그 목록 탐색
- `Space (특정 태그에서)`: 해당 태그 선택/해제
- `Ctrl+N`: 현재 입력 텍스트로 새 태그 생성
- `Backspace`: 검색어 수정
- `문자 추가`: 검색어 확장, 실시간 필터링

### Technical Implementation Details

#### State Management
```rust
enum TagInputState {
    Empty,                    // Case 1
    NewTagInput(String),      // Case 2  
    TagSelected(Vec<String>), // Case 2-2
    TagSearch {               // Case 3
        query: String,
        matches: Vec<String>,
        selected: Vec<String>,
    },
}
```

#### Key Bindings
- `Enter`: 현재 상태에 따른 완료 액션
- `Space`: 태그 선택/해제 또는 브라우저 모드 진입
- `↑/↓`: 태그 목록 탐색 (Case 3에서)
- `Ctrl+N`: 강제 새 태그 생성 (Case 3에서)
- `Backspace`: 문자 삭제 또는 이전 상태로 복귀
- `Ctrl+C`: 태그 선택 취소, 프로젝트 생성 중단

#### Fuzzy Matching Algorithm
- **Exact match priority**: 정확히 일치하는 태그가 최우선
- **Prefix match**: 입력으로 시작하는 태그들이 다음 우선순위
- **Contains match**: 입력을 포함하는 태그들이 마지막 우선순위
- **Case insensitive**: 대소문자 구분 없이 매칭

#### Visual Indicators
- `[✓]`: 선택된 태그
- `[ ]`: 선택 가능한 태그
- `(Enter to ...)`: 현재 Enter 키의 동작 설명
- `🏷️  Tags:`: 태그 입력 프롬프트
- 실시간 매칭 카운트: `(3 matches found)`

### Example User Flows

#### Flow 1: 태그 없이 프로젝트 생성
```
🏷️  Tags: 
(Enter to create project without tags)
→ [User presses Enter]
✅ Created project 'my-app' without tags
```

#### Flow 2: 새 태그 생성
```
🏷️  Tags: 
(Enter to create project without tags)
→ [User types: "react"]

🏷️  Tags: react
(Enter to create tag `react`)
→ [User presses Enter]

🏷️  Tags: 
(Enter to create project with selected tags)
[✓] react
→ [User presses Enter]
✅ Created project 'my-app' with tags: react
```

#### Flow 3: 기존 태그 검색 및 선택
```
🏷️  Tags: 
(Enter to create project without tags)
→ [User types: "r"]

🏷️  Tags: r
(Enter to create project with selected tags)
[ ] react
[ ] rust
[ ] ruby
→ [User presses Space to select first item]

🏷️  Tags: r
(Enter to create project with selected tags)
[✓] react
[ ] rust  
[ ] ruby
→ [User presses Enter]
✅ Created project 'my-app' with tags: react
```

### Implementation Priority

1. **Phase 1**: State machine 구현 및 기본 UI 플로우
2. **Phase 2**: Fuzzy matching 알고리즘 구현
3. **Phase 3**: 키보드 네비게이션 및 고급 기능
4. **Phase 4**: 사용성 테스트 및 최적화

### Testing Scenarios

1. **Empty input handling**: 아무것도 입력하지 않고 Enter
2. **New tag creation**: 존재하지 않는 태그 이름 입력
3. **Existing tag search**: 기존 태그와 부분 일치하는 검색
4. **Multiple tag selection**: 여러 태그 선택 및 해제
5. **Edge cases**: 매우 긴 태그 이름, 특수 문자, 중복 이름 등

이 스펙을 바탕으로 현재의 태그 선택 인터페이스를 개선하여 더욱 직관적이고 효율적인 사용자 경험을 제공할 수 있습니다.