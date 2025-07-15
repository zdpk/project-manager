# PM Starship Integration Guide

이 가이드에서는 PM(Project Manager)을 Starship 프롬프트와 연동하여 현재 디렉토리의 프로젝트 정보를 터미널 프롬프트에 표시하는 방법을 설명합니다.

## 개요

PM의 `status` 명령어를 사용하여 Starship에서 현재 디렉토리가 PM으로 관리되는 프로젝트인지 감지하고, 프로젝트 이름, 태그, Git 상태 등의 정보를 프롬프트에 표시할 수 있습니다.

이 가이드는 PM과 Starship을 수동으로 연동하는 방법을 단계별로 안내합니다.

## 📋 Quick Setup (빠른 설정)

가장 간단한 방법으로 PM과 Starship을 연동하는 방법입니다.

### Prerequisites

1. **Starship 설치 확인**:
   ```bash
   starship --version
   # 설치되지 않은 경우:
   curl -sS https://starship.rs/install.sh | sh
   ```

2. **PM 설치 확인**:
   ```bash
   pm --version
   # PM이 설치되어 있어야 합니다
   ```

3. **jq 설치 확인** (JSON 파싱용):
   ```bash
   jq --version
   # 설치되지 않은 경우:
   # macOS: brew install jq
   # Ubuntu: sudo apt-get install jq
   # CentOS: sudo yum install jq
   ```

### Step-by-Step Setup

```bash
# 1. Starship 설정 파일 열기
nano ~/.config/starship.toml
# 또는
vim ~/.config/starship.toml

# 2. 다음 설정을 파일 끝에 추가:
[custom.pm]
command = '''pm status --format json --quiet | jq -r "
  if .git_branch != \"\" then
    if .git_changes then .name + \" [\" + .git_branch + \"*]\"
    else .name + \" [\" + .git_branch + \"]\"
    end
  else .name
  end
" 2>/dev/null || echo ""'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project with git status"

# 3. 설정 테스트
pm status --format json --quiet

# 4. 쉘 재시작 또는 설정 다시 로드
exec $SHELL
# 또는
source ~/.config/starship.toml
```

## 🔧 Development Environment Setup (개발 환경)

로컬에서 빌드한 PM 바이너리를 사용하는 개발 환경에서의 설정 절차입니다.

### Prerequisites

1. **Rust 개발 환경**:
   ```bash
   rustc --version
   cargo --version
   ```

2. **PM 프로젝트 클론 및 빌드**:
   ```bash
   git clone https://github.com/zdpk/project-manager.git
   cd project-manager
   cargo build --release
   ```

3. **Starship 설치**:
   ```bash
   curl -sS https://starship.rs/install.sh | sh
   ```

### Development Binary Setup

```bash
# 1. 개발 바이너리 경로 설정
export _PM_BINARY="/path/to/project-manager/target/release/pm"

# 영구적으로 설정하려면 쉘 설정 파일에 추가:
# ~/.bashrc, ~/.zshrc, 또는 ~/.config/fish/config.fish
echo 'export _PM_BINARY="/path/to/project-manager/target/release/pm"' >> ~/.zshrc

# 2. 개발 바이너리로 설정 테스트
$_PM_BINARY status --format json --quiet

# 또는 PATH를 임시로 수정
PATH="/path/to/project-manager/target/release:$PATH" pm status
```

### Development Workflow

```bash
# 1. 코드 변경 후 다시 빌드
cargo build --release

# 2. 새로운 상태 출력 테스트
$_PM_BINARY status --format json --quiet

# 3. Starship 설정 파일 수정 (필요한 경우)
# ~/.config/starship.toml 파일에서 command 부분을 개발 바이너리로 변경:
[custom.pm]
command = '''$_PM_BINARY status --format json --quiet | jq -r "..."'''
when = "$_PM_BINARY status --quiet"
format = "📁 [$output](bold blue) "
```

### Development Binary Integration

개발 환경에서 쉘 통합을 위한 설정:

```bash
# ~/.zshrc 또는 ~/.bashrc에 추가
export _PM_BINARY="/path/to/project-manager/target/release/pm"

# PM 개발 함수 (선택사항)
pm_dev() {
    $_PM_BINARY "$@"
}

# 개발 중 빠른 테스트를 위한 별칭
alias pm-status="$_PM_BINARY status --format json --quiet"
alias pm-status-test="$_PM_BINARY status"
```

### Multi-Version Testing

여러 PM 버전을 동시에 테스트하는 경우:

```bash
# 각 버전별 별칭 설정
alias pm-main="/path/to/pm-main/target/release/pm"
alias pm-dev="/path/to/pm-dev/target/release/pm"
alias pm-feature="/path/to/pm-feature/target/release/pm"

# 각 버전별 상태 출력 테스트
pm-main status --format json --quiet
pm-dev status --format json --quiet
pm-feature status --format json --quiet
```

## PM Status 명령어

### 기본 사용법

```bash
# 현재 디렉토리의 프로젝트 상태 표시
pm status

# 출력 예시:
📋 Project: project-manager
🏷️  Tags: rust, cli, tools
📁 Path: /Users/user/github/project-manager
🌿 Git: feat/enhanced-add-command (with changes)
📊 Access count: 15
🕒 Last accessed: 2025-07-15 10:30:00
```

### Starship 연동용 옵션

```bash
# JSON 형식으로 출력 (Starship에서 파싱하기 쉬움)
pm status --format json

# 프롬프트용 간단한 출력
pm status --quiet

# JSON + quiet 모드 (가장 컴팩트한 JSON 출력)
pm status --format json --quiet
```

### 출력 형식

#### Text 형식 (기본)
```bash
$ pm status
📋 Project: project-manager
🏷️  Tags: rust, cli, tools
📁 Path: /Users/user/github/project-manager
🌿 Git: feat/enhanced-add-command (with changes)
📊 Access count: 15
🕒 Last accessed: 2025-07-15 10:30:00
```

#### Text Quiet 형식
```bash
$ pm status --quiet
project-manager (rust, cli, tools) [feat/enhanced-add-command*]
```

#### JSON 형식
```json
{
  "project": {
    "name": "project-manager",
    "tags": ["rust", "cli", "tools"],
    "path": "/Users/user/github/project-manager",
    "description": "CLI project manager",
    "language": "Rust"
  },
  "git": {
    "is_repository": true,
    "branch": "feat/enhanced-add-command",
    "has_changes": true,
    "remote_url": "https://github.com/user/project-manager.git",
    "last_commit": "2025-07-15T10:00:00Z"
  },
  "metadata": {
    "access_count": 15,
    "last_accessed": "2025-07-15T10:30:00Z"
  }
}
```

#### JSON Quiet 형식
```json
{
  "name": "project-manager",
  "tags": "rust,cli,tools", 
  "git_branch": "feat/enhanced-add-command",
  "git_changes": true
}
```

## Starship 설정

### 기본 설정

`~/.config/starship.toml` 파일에 다음을 추가하세요:

```toml
[custom.pm]
command = "pm status --format json --quiet"
when = "pm status --quiet"
format = "[$output]($style) "
style = "bold blue"
description = "Show PM project information"
```

### 고급 설정

더 세밀한 제어를 원한다면:

```toml
[custom.pm_project]
command = '''bash -c "
  if pm status --quiet >/dev/null 2>&1; then
    name=$(pm status --format json --quiet | jq -r '.name')
    tags=$(pm status --format json --quiet | jq -r '.tags')
    branch=$(pm status --format json --quiet | jq -r '.git_branch')
    changes=$(pm status --format json --quiet | jq -r '.git_changes')
    
    output=\"📁 $name\"
    if [[ \"$tags\" != \"\" ]]; then
      output=\"$output ($tags)\"
    fi
    if [[ \"$branch\" != \"\" ]]; then
      if [[ \"$changes\" == \"true\" ]]; then
        output=\"$output [$branch*]\"
      else
        output=\"$output [$branch]\"
      fi
    fi
    echo \"$output\"
  fi
"'''
when = "pm status --quiet"
format = "[$output]($style) "
style = "bold cyan"
shell = ["bash", "--noprofile", "--norc"]
```

### 조건부 표시 설정

프로젝트가 있을 때만 표시:

```toml
[custom.pm]
command = "pm status --format json --quiet"
when = "pm status --quiet"
format = "via [$output]($style) "
style = "bold blue"
```

### 여러 정보 표시

```toml
# 프로젝트 이름만 표시
[custom.pm_name]
command = 'pm status --format json --quiet | jq -r ".name"'
when = "pm status --quiet"
format = "📁 [$output]($style) "
style = "bold blue"

# Git 브랜치와 변경사항 표시
[custom.pm_git]
command = '''bash -c "
  if pm status --quiet >/dev/null 2>&1; then
    json=$(pm status --format json --quiet)
    branch=$(echo $json | jq -r '.git_branch')
    changes=$(echo $json | jq -r '.git_changes')
    if [[ \"$branch\" != \"\" && \"$branch\" != \"null\" ]]; then
      if [[ \"$changes\" == \"true\" ]]; then
        echo \"$branch*\"
      else
        echo \"$branch\"
      fi
    fi
  fi
"'''
when = "pm status --quiet"
format = "🌿 [$output]($style) "
style = "bold green"

# 태그 표시
[custom.pm_tags]
command = 'pm status --format json --quiet | jq -r ".tags" | sed "s/,/, /g"'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".tags") != "" ]]'
format = "🏷️  [$output]($style) "
style = "bold yellow"
```

## 성능 고려사항

### 캐싱

프롬프트 성능을 위해 `pm status` 결과를 캐싱할 수 있습니다:

```toml
[custom.pm]
command = '''bash -c "
  cache_file=\"/tmp/pm_status_$(pwd | sed 's/\//_/g')\"
  if [[ -f \"$cache_file\" && $(find \"$cache_file\" -mmin -1) ]]; then
    cat \"$cache_file\"
  else
    if result=$(pm status --format json --quiet 2>/dev/null); then
      echo \"$result\" | tee \"$cache_file\"
    fi
  fi
"'''
when = "pm status --quiet"
format = "[$output]($style) "
style = "bold blue"
```

### 타임아웃 설정

```toml
[custom.pm]
command = "timeout 0.1s pm status --format json --quiet"
when = "timeout 0.1s pm status --quiet"
format = "[$output]($style) "
style = "bold blue"
```

## 스타일링 예시

### 미니멀 스타일
```toml
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name"'
when = "pm status --quiet"
format = "[$output]($style) "
style = "dimmed blue"
```

### 상세 스타일
```toml
[custom.pm]
command = '''bash -c "
  if json=$(pm status --format json --quiet 2>/dev/null); then
    name=$(echo $json | jq -r '.name')
    tags=$(echo $json | jq -r '.tags')
    branch=$(echo $json | jq -r '.git_branch')
    changes=$(echo $json | jq -r '.git_changes')
    
    output=\"📁 $name\"
    if [[ \"$tags\" != \"\" ]]; then
      output=\"$output 🏷️ $tags\"
    fi
    if [[ \"$branch\" != \"\" ]]; then
      if [[ \"$changes\" == \"true\" ]]; then
        output=\"$output 🌿 $branch*\"
      else
        output=\"$output 🌿 $branch\"
      fi
    fi
    echo \"$output\"
  fi
"'''
when = "pm status --quiet"
format = "[$output]($style) "
style = "bold cyan"
```

### 컬러 코딩
```toml
[custom.pm_project]
command = 'pm status --format json --quiet | jq -r ".name"'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

[custom.pm_git_clean]
command = 'pm status --format json --quiet | jq -r ".git_branch"'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".git_changes") == "false" ]]'
format = "🌿 [$output](bold green) "

[custom.pm_git_dirty]
command = 'pm status --format json --quiet | jq -r ".git_branch"'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".git_changes") == "true" ]]'
format = "🌿 [$output*](bold red) "
```

## 문제 해결

### 성능 문제
- `timeout` 명령어를 사용하여 `pm status` 실행 시간을 제한하세요
- 캐싱을 사용하여 반복적인 호출을 줄이세요
- `pm status --quiet` 모드를 사용하여 출력을 최소화하세요

### JSON 파싱 오류
- `jq`가 설치되어 있는지 확인하세요: `brew install jq` (macOS) 또는 `apt-get install jq` (Ubuntu)
- JSON 출력이 유효한지 확인하세요: `pm status --format json --quiet | jq .`

### 프로젝트 감지 실패
- 현재 디렉토리가 PM 프로젝트인지 확인하세요: `pm status`
- 상위 디렉토리 감지가 작동하는지 확인하세요

## 예시 설정 모음

### 간단한 설정
```toml
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name"'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

### 중간 복잡도 설정
```toml
[custom.pm]
command = '''pm status --format json --quiet | jq -r "
  if .git_branch != \"\" then
    if .git_changes then .name + \" [\" + .git_branch + \"*]\"
    else .name + \" [\" + .git_branch + \"]\"
    end
  else .name
  end
"'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

### 완전한 설정
```toml
[custom.pm]
command = '''bash -c "
  if json=$(pm status --format json --quiet 2>/dev/null); then
    name=$(echo $json | jq -r '.name')
    tags=$(echo $json | jq -r '.tags')
    branch=$(echo $json | jq -r '.git_branch')
    changes=$(echo $json | jq -r '.git_changes')
    
    # Base output with project name
    output=\"$name\"
    
    # Add tags if present
    if [[ \"$tags\" != \"\" && \"$tags\" != \"null\" ]]; then
      output=\"$output ($tags)\"
    fi
    
    # Add git info if present
    if [[ \"$branch\" != \"\" && \"$branch\" != \"null\" ]]; then
      if [[ \"$changes\" == \"true\" ]]; then
        output=\"$output [$branch*]\"
      else
        output=\"$output [$branch]\"
      fi
    fi
    
    echo \"$output\"
  fi
"'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
style = "bold blue"
```

## 🔧 Troubleshooting

### Common Issues and Solutions

#### 1. Starship Not Installed

**문제**: Starship이 설치되지 않음
```bash
$ starship --version
zsh: command not found: starship
```

**해결방법**:
```bash
# macOS/Linux
curl -sS https://starship.rs/install.sh | sh

# 설치 확인
starship --version

# 쉘 설정에 Starship 초기화 추가 (아직 하지 않은 경우)
# Bash
echo 'eval "$(starship init bash)"' >> ~/.bashrc

# Zsh
echo 'eval "$(starship init zsh)"' >> ~/.zshrc

# Fish
echo 'starship init fish | source' >> ~/.config/fish/config.fish
```

#### 2. jq Command Not Found

**문제**: JSON 파싱에 필요한 `jq`가 설치되지 않음
```bash
$ pm status --format json --quiet | jq -r ".name"
zsh: command not found: jq
```

**해결방법**:
```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# CentOS/RHEL
sudo yum install jq

# jq 설치 확인
jq --version
```

#### 3. PM Configuration Not Found

**문제**: PM이 초기화되지 않음
```bash
$ pm status
PM not initialized
Configuration file not found

💡 Please initialize PM first:
   pm init
```

**해결방법**:
```bash
# PM 초기화
pm init

# 기존 프로젝트 추가
pm add .

# 또는 프로젝트 스캔
pm scan
```

#### 4. PM Module Not Showing in Prompt

**문제**: Starship 설정을 추가했지만 프롬프트에 나타나지 않음

**해결방법**:
```bash
# 1. 현재 디렉토리가 PM 프로젝트인지 확인
pm status

# 2. PM 상태 출력 테스트
pm status --format json --quiet

# 3. Starship 설정 파일 확인
cat ~/.config/starship.toml | grep -A 10 "\[custom.pm\]"

# 4. 쉘 재시작
exec $SHELL

# 5. Starship 다시 로드
source ~/.config/starship.toml
```

#### 5. Performance Issues (Slow Prompt)

**문제**: 프롬프트가 느려짐

**해결방법**:
```bash
# 1. 타임아웃 설정 추가
[custom.pm]
command = "timeout 0.5s pm status --format json --quiet"
when = "timeout 0.1s pm status --quiet"
format = "📁 [$output](bold blue) "

# 2. 캐싱 사용
[custom.pm]
command = '''bash -c "
  cache_file=\"/tmp/pm_status_$(pwd | sed 's/\//_/g')\"
  if [[ -f \"$cache_file\" && $(find \"$cache_file\" -mmin -1) ]]; then
    cat \"$cache_file\"
  else
    if result=$(pm status --format json --quiet 2>/dev/null); then
      echo \"$result\" | tee \"$cache_file\"
    fi
  fi
"'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

#### 6. Development Binary Issues

**문제**: 개발 바이너리가 올바르게 인식되지 않음

**해결방법**:
```bash
# 1. _PM_BINARY 환경변수 확인
echo $_PM_BINARY

# 2. 바이너리 실행 권한 확인
ls -la $_PM_BINARY

# 3. 바이너리가 작동하는지 테스트
$_PM_BINARY --version

# 4. 개발 바이너리 재빌드
cd /path/to/project-manager
cargo build --release

# 5. 쉘 설정에 환경변수 추가
echo 'export _PM_BINARY="/path/to/project-manager/target/release/pm"' >> ~/.zshrc
source ~/.zshrc
```

#### 7. JSON Output Parsing Errors

**문제**: JSON 파싱 중 오류 발생

**해결방법**:
```bash
# 1. PM 상태 출력 확인
pm status --format json --quiet

# 2. JSON 유효성 검증
pm status --format json --quiet | jq .

# 3. jq 없이 사용하는 간단한 설정으로 변경
[custom.pm]
command = 'pm status --format json --quiet'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

# 4. 또는 텍스트 출력 사용
[custom.pm]
command = 'pm status --quiet'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

### Debug Commands

문제 진단을 위한 유용한 명령어들:

```bash
# 1. 전체 환경 확인
echo "PM Version: $(pm --version)"
echo "Starship Version: $(starship --version)"
echo "Shell: $SHELL"
echo "jq Available: $(command -v jq || echo 'Not installed')"
echo "_PM_BINARY: $_PM_BINARY"

# 2. PM 상태 상세 확인
pm status
pm status --format json
pm status --format json --quiet

# 3. Starship 설정 파일 확인
cat ~/.config/starship.toml | grep -A 10 "\[custom.pm\]"

# 4. 개발 바이너리 확인 (해당되는 경우)
ls -la $_PM_BINARY
$_PM_BINARY --version

```

### Getting Help

추가 도움이 필요한 경우:

1. **GitHub Issues**: [https://github.com/zdpk/project-manager/issues](https://github.com/zdpk/project-manager/issues)
2. **Documentation**: [README.md](../README.md) 및 [COMMANDS.md](COMMANDS.md)
3. **Command Help**: `pm --help`, `pm starship --help`

## 🎨 Configuration Examples

### Quick Reference

다양한 사용 시나리오에 맞는 설정 예제들:

```toml
# 1. 최소 설정 (프로젝트 이름만)
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

# 2. 기본 설정 (프로젝트 이름 + Git)
[custom.pm]
command = '''pm status --format json --quiet | jq -r "
  if .git_branch != \"\" then
    if .git_changes then .name + \" [\" + .git_branch + \"*]\"
    else .name + \" [\" + .git_branch + \"]\"
    end
  else .name
  end
" 2>/dev/null || echo ""'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

# 3. 아이콘 없는 설정
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "[PM: $output](bold blue) "

# 4. 녹색 테마
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold green) "

# 5. 간단한 텍스트 출력 (jq 없이)
[custom.pm]
command = 'pm status --quiet'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
```

이제 Starship 프롬프트에서 PM 프로젝트 정보를 확인할 수 있습니다! 🚀