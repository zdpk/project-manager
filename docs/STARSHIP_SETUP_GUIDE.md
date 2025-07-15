# PM Starship Setup Guide

이 가이드는 PM(Project Manager)과 Starship을 연동하여 터미널 프롬프트에 프로젝트 정보를 표시하는 방법을 단계별로 안내합니다.

## 🚀 Quick Setup (빠른 설정)

가장 간단한 방법으로 PM Starship 연동을 설정하세요:

```bash
# 1. PM 설치 확인
pm --version  # 0.1.1 이상 필요

# 2. Starship 설치 (설치되지 않은 경우)
curl -sS https://starship.rs/install.sh | sh

# 3. PM Starship 도우미 실행
pm starship

# 4. 인터랙티브 설정 완료 후 쉘 재시작
exec $SHELL
```

완료! 이제 PM 프로젝트 디렉토리에서 터미널 프롬프트에 프로젝트 정보가 표시됩니다.

## 📋 Production Environment Setup (프로덕션 환경)

### Prerequisites

시스템에 설치된 PM 바이너리를 사용하는 경우:

```bash
# 1. PM 설치 확인
pm --version
# 출력: pm 0.1.1

# 2. Starship 설치 확인
starship --version
# 출력: starship 1.16.0

# 3. jq 설치 확인 (JSON 파싱용)
jq --version
# 출력: jq-1.6

# 설치되지 않은 경우:
# macOS: brew install jq
# Ubuntu: sudo apt-get install jq
# CentOS: sudo yum install jq
```

### Step-by-Step Setup

#### 1단계: PM 설치

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/zdpk/project-manager/releases/latest/download/install.sh | sh

# 수동 설치
curl -L https://github.com/zdpk/project-manager/releases/latest/download/pm-aarch64-apple-darwin -o pm
chmod +x pm
sudo mv pm /usr/local/bin/

# 설치 확인
pm --version
```

#### 2단계: Starship 설치

```bash
# Starship 설치
curl -sS https://starship.rs/install.sh | sh

# 쉘 설정에 Starship 초기화 추가
# Bash
echo 'eval "$(starship init bash)"' >> ~/.bashrc

# Zsh
echo 'eval "$(starship init zsh)"' >> ~/.zshrc

# Fish
echo 'starship init fish | source' >> ~/.config/fish/config.fish

# 설정 다시 로드
source ~/.bashrc  # 또는 ~/.zshrc
```

#### 3단계: PM Starship 연동 설정

```bash
# PM Starship 도우미 실행
pm starship

# 인터랙티브 설정 과정:
# 1. 스타일 선택 (minimal, basic, detailed)
# 2. 아이콘 사용 여부 선택
# 3. 컬러 테마 선택
# 4. 설정이 클립보드에 복사됨
```

#### 4단계: Starship 설정 파일 편집

```bash
# 설정 파일 열기
nano ~/.config/starship.toml
# 또는
vim ~/.config/starship.toml

# 클립보드에서 복사한 설정을 파일 끝에 붙여넣기
# 예시:
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
```

#### 5단계: 설정 테스트 및 쉘 재시작

```bash
# 설정 테스트
pm starship --test

# 쉘 재시작
exec $SHELL

# 또는 설정 다시 로드
source ~/.config/starship.toml
```

### Automated Setup Script

프로덕션 환경에서 자동 설정을 위한 스크립트:

```bash
#!/bin/bash
# pm-starship-setup.sh - PM Starship 자동 설정 스크립트

set -e

echo "🚀 PM Starship Integration Setup"
echo "================================="

# 1. PM 설치 확인
if ! command -v pm &> /dev/null; then
    echo "📦 Installing PM..."
    curl -fsSL https://github.com/zdpk/project-manager/releases/latest/download/install.sh | sh
    echo "✅ PM installed"
fi

# 2. Starship 설치 확인
if ! command -v starship &> /dev/null; then
    echo "🌟 Installing Starship..."
    curl -sS https://starship.rs/install.sh | sh
    echo "✅ Starship installed"
fi

# 3. jq 설치 확인
if ! command -v jq &> /dev/null; then
    echo "🔧 Installing jq..."
    if command -v brew &> /dev/null; then
        brew install jq
    elif command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y jq
    elif command -v yum &> /dev/null; then
        sudo yum install -y jq
    else
        echo "⚠️  Please install jq manually"
    fi
    echo "✅ jq installed"
fi

# 4. Starship 설정 디렉토리 생성
mkdir -p ~/.config

# 5. PM Starship 설정 생성
echo "⚙️  Generating PM Starship configuration..."
pm starship --style basic --show >> ~/.config/starship.toml

# 6. 쉘 설정에 Starship 초기화 추가
SHELL_NAME=$(basename "$SHELL")
case "$SHELL_NAME" in
    bash)
        if ! grep -q "starship init bash" ~/.bashrc; then
            echo 'eval "$(starship init bash)"' >> ~/.bashrc
            echo "✅ Added Starship to ~/.bashrc"
        fi
        ;;
    zsh)
        if ! grep -q "starship init zsh" ~/.zshrc; then
            echo 'eval "$(starship init zsh)"' >> ~/.zshrc
            echo "✅ Added Starship to ~/.zshrc"
        fi
        ;;
    fish)
        if ! grep -q "starship init fish" ~/.config/fish/config.fish; then
            echo 'starship init fish | source' >> ~/.config/fish/config.fish
            echo "✅ Added Starship to ~/.config/fish/config.fish"
        fi
        ;;
esac

echo ""
echo "🎉 Setup complete!"
echo "📝 Please restart your shell: exec \$SHELL"
echo "💡 Test your setup: pm starship --test"
```

사용법:
```bash
curl -fsSL https://raw.githubusercontent.com/zdpk/project-manager/main/scripts/pm-starship-setup.sh | bash
```

## 🔧 Development Environment Setup (개발 환경)

### Prerequisites

로컬에서 PM을 빌드하고 개발하는 경우:

```bash
# 1. Rust 개발 환경 확인
rustc --version
cargo --version

# 2. PM 프로젝트 클론
git clone https://github.com/zdpk/project-manager.git
cd project-manager

# 3. PM 빌드
cargo build --release

# 4. 빌드 확인
./target/release/pm --version
```

### Development Binary Setup

#### 환경 변수 설정

```bash
# 1. 개발 바이너리 경로 설정
export _PM_BINARY="$(pwd)/target/release/pm"

# 2. 영구 설정 (선택사항)
# ~/.bashrc 또는 ~/.zshrc에 추가
echo "export _PM_BINARY=\"$(pwd)/target/release/pm\"" >> ~/.zshrc

# 3. 설정 확인
echo $_PM_BINARY
$_PM_BINARY --version
```

#### 개발 환경 Starship 설정

```bash
# 1. 개발 바이너리로 설정 생성
$_PM_BINARY starship

# 2. 또는 PATH를 임시로 수정
PATH="$(pwd)/target/release:$PATH" pm starship

# 3. 설정 테스트
$_PM_BINARY starship --test
```

### Development Workflow

```bash
# 1. 코드 수정 후 재빌드
cargo build --release

# 2. 새로운 기능 테스트
$_PM_BINARY starship --test

# 3. 다른 스타일 테스트
$_PM_BINARY starship --style minimal --show
$_PM_BINARY starship --style basic --show
$_PM_BINARY starship --style detailed --show

# 4. 설정 업데이트
$_PM_BINARY starship --show > /tmp/pm-config.toml
cat /tmp/pm-config.toml >> ~/.config/starship.toml
```

### Development Environment Script

개발 환경 설정 자동화 스크립트:

```bash
#!/bin/bash
# pm-dev-setup.sh - PM 개발 환경 Starship 설정

set -e

echo "🔧 PM Development Environment Setup"
echo "=================================="

# 1. PM 프로젝트 디렉토리 확인
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    echo "❌ Please run this script from the PM project root directory"
    exit 1
fi

# 2. PM 빌드
echo "🔨 Building PM..."
cargo build --release

# 3. 개발 바이너리 경로 설정
DEV_BINARY="$(pwd)/target/release/pm"
echo "📍 Development binary: $DEV_BINARY"

# 4. 환경 변수 설정
export _PM_BINARY="$DEV_BINARY"

# 5. Starship 설치 확인
if ! command -v starship &> /dev/null; then
    echo "🌟 Installing Starship..."
    curl -sS https://starship.rs/install.sh | sh
fi

# 6. 개발 바이너리로 설정 생성
echo "⚙️  Generating development configuration..."
$_PM_BINARY starship --style basic --show

# 7. 쉘 설정에 환경 변수 추가 (선택사항)
read -p "Add _PM_BINARY to shell config? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
        bash)
            echo "export _PM_BINARY=\"$DEV_BINARY\"" >> ~/.bashrc
            echo "✅ Added _PM_BINARY to ~/.bashrc"
            ;;
        zsh)
            echo "export _PM_BINARY=\"$DEV_BINARY\"" >> ~/.zshrc
            echo "✅ Added _PM_BINARY to ~/.zshrc"
            ;;
        fish)
            echo "set -x _PM_BINARY \"$DEV_BINARY\"" >> ~/.config/fish/config.fish
            echo "✅ Added _PM_BINARY to ~/.config/fish/config.fish"
            ;;
    esac
fi

echo ""
echo "🎉 Development setup complete!"
echo "💡 Test your setup: \$_PM_BINARY starship --test"
```

사용법:
```bash
cd /path/to/project-manager
./scripts/pm-dev-setup.sh
```

### Multi-Version Testing

여러 PM 버전을 동시에 테스트하는 경우:

```bash
# 1. 각 버전별 별칭 설정
alias pm-main="/path/to/pm-main/target/release/pm"
alias pm-dev="/path/to/pm-dev/target/release/pm"
alias pm-feature="/path/to/pm-feature/target/release/pm"

# 2. 각 버전별 설정 생성
pm-main starship --style basic --show > /tmp/pm-main-config.toml
pm-dev starship --style basic --show > /tmp/pm-dev-config.toml
pm-feature starship --style basic --show > /tmp/pm-feature-config.toml

# 3. 각 버전별 테스트
pm-main starship --test
pm-dev starship --test
pm-feature starship --test

# 4. 설정 비교
diff /tmp/pm-main-config.toml /tmp/pm-dev-config.toml
```

## 📊 Advanced Configuration (고급 설정)

### Configuration Styles

#### 1. Minimal Style (최소 스타일)

프로젝트 이름만 표시:

```toml
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project name"
```

출력 예시: `📁 project-manager`

#### 2. Basic Style (기본 스타일)

프로젝트 이름 + Git 브랜치:

```toml
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
```

출력 예시: `📁 project-manager [main*]`

#### 3. Detailed Style (상세 스타일)

개별 모듈로 분리된 정보:

```toml
# 프로젝트 이름
[custom.pm_project]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

# 태그
[custom.pm_tags]
command = 'pm status --format json --quiet | jq -r ".tags" 2>/dev/null | sed "s/,/, /g"'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".tags" 2>/dev/null) != "" ]]'
format = "🏷️  [$output](bold yellow) "

# Git 상태 (변경사항 없음)
[custom.pm_git_clean]
command = 'pm status --format json --quiet | jq -r ".git_branch" 2>/dev/null || echo ""'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".git_changes" 2>/dev/null) == "false" ]]'
format = "🌿 [$output](bold green) "

# Git 상태 (변경사항 있음)
[custom.pm_git_dirty]
command = 'pm status --format json --quiet | jq -r ".git_branch" 2>/dev/null || echo ""'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".git_changes" 2>/dev/null) == "true" ]]'
format = "🌿 [$output*](bold red) "
```

출력 예시: `📁 project-manager 🏷️ rust, cli 🌿 main*`

### Custom Configuration

#### 아이콘 없는 설정

```toml
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "[PM: $output](bold blue) "
description = "Show PM project name without icons"
```

#### 다른 컬러 테마

```toml
# 녹색 테마
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold green) "

# 보라색 테마
[custom.pm]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold purple) "

# 컬러풀 테마
[custom.pm_project]
command = 'pm status --format json --quiet | jq -r ".name" 2>/dev/null || echo ""'
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

[custom.pm_tags]
command = 'pm status --format json --quiet | jq -r ".tags" 2>/dev/null | sed "s/,/, /g"'
when = 'pm status --quiet && [[ $(pm status --format json --quiet | jq -r ".tags" 2>/dev/null) != "" ]]'
format = "🏷️  [$output](bold yellow) "
```

### Performance Optimization

#### 타임아웃 설정

```toml
[custom.pm]
command = "timeout 0.5s pm status --format json --quiet"
when = "timeout 0.1s pm status --quiet"
format = "📁 [$output](bold blue) "
description = "Show PM project with timeout"
```

#### 캐싱 설정

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
format = "📁 [$output](bold blue) "
description = "Show PM project with caching"
```

## 🔍 Troubleshooting (문제 해결)

### Common Issues

#### 1. PM 명령어를 찾을 수 없음

```bash
# 문제
$ pm starship
zsh: command not found: pm

# 해결
# PM 설치 확인
which pm

# PM 설치
curl -fsSL https://github.com/zdpk/project-manager/releases/latest/download/install.sh | sh

# PATH 확인
echo $PATH

# 쉘 재시작
exec $SHELL
```

#### 2. Starship이 설치되지 않음

```bash
# 문제
$ pm starship --test
❌ Starship is not installed

# 해결
# Starship 설치
curl -sS https://starship.rs/install.sh | sh

# 쉘 설정에 추가
echo 'eval "$(starship init zsh)"' >> ~/.zshrc
source ~/.zshrc
```

#### 3. jq 명령어를 찾을 수 없음

```bash
# 문제
$ pm status --format json --quiet | jq -r ".name"
zsh: command not found: jq

# 해결
# macOS
brew install jq

# Ubuntu
sudo apt-get install jq

# CentOS
sudo yum install jq

# 또는 jq 없이 사용
pm starship --style minimal
```

#### 4. 프롬프트에 PM 정보가 표시되지 않음

```bash
# 진단
# 1. PM 상태 확인
pm status

# 2. Starship 설정 확인
pm starship --test

# 3. 설정 파일 확인
cat ~/.config/starship.toml | grep -A 10 "\[custom.pm\]"

# 4. 설정 다시 로드
source ~/.config/starship.toml
exec $SHELL
```

#### 5. 개발 바이너리 인식 실패

```bash
# 문제
$ $_PM_BINARY starship
zsh: no such file or directory: /path/to/pm

# 해결
# 1. 바이너리 경로 확인
echo $_PM_BINARY
ls -la $_PM_BINARY

# 2. 재빌드
cd /path/to/project-manager
cargo build --release

# 3. 권한 확인
chmod +x $_PM_BINARY

# 4. 환경 변수 재설정
export _PM_BINARY="/path/to/project-manager/target/release/pm"
```

### Debug Commands

문제 진단을 위한 명령어:

```bash
# 1. 환경 정보 확인
echo "PM Version: $(pm --version)"
echo "Starship Version: $(starship --version)"
echo "Shell: $SHELL"
echo "jq Available: $(command -v jq || echo 'Not installed')"
echo "_PM_BINARY: $_PM_BINARY"

# 2. PM 상태 확인
pm status
pm status --format json
pm status --format json --quiet

# 3. Starship 설정 테스트
pm starship --test

# 4. 설정 파일 확인
cat ~/.config/starship.toml | grep -A 20 "\[custom.pm"

# 5. 프롬프트 테스트
starship prompt
```

### Performance Issues

프롬프트 속도가 느린 경우:

```bash
# 1. 타임아웃 설정 추가
[custom.pm]
command = "timeout 0.3s pm status --format json --quiet"
when = "timeout 0.1s pm status --quiet"
format = "📁 [$output](bold blue) "

# 2. 캐시 설정
[custom.pm]
command = '''bash -c "
  cache_file=\"/tmp/pm_status_$(pwd | tr '/' '_')\"
  if [[ -f \"$cache_file\" && $(find \"$cache_file\" -mmin -1) ]]; then
    cat \"$cache_file\"
  else
    pm status --format json --quiet | tee \"$cache_file\"
  fi
"'''
when = "pm status --quiet"
format = "📁 [$output](bold blue) "

# 3. 최소 설정 사용
pm starship --style minimal
```

## 🎯 Best Practices (모범 사례)

### Configuration Management

1. **설정 백업**:
   ```bash
   # Starship 설정 백업
   cp ~/.config/starship.toml ~/.config/starship.toml.backup
   
   # PM 설정 백업
   pm config backup create starship-setup
   ```

2. **버전 관리**:
   ```bash
   # 설정 파일을 Git으로 관리
   git add ~/.config/starship.toml
   git commit -m "Add PM Starship configuration"
   ```

3. **환경별 설정**:
   ```bash
   # 개발 환경
   [custom.pm_dev]
   command = '$_PM_BINARY status --format json --quiet | jq -r ".name"'
   when = '$_PM_BINARY status --quiet'
   format = "🔧 [$output](bold yellow) "
   
   # 프로덕션 환경
   [custom.pm_prod]
   command = 'pm status --format json --quiet | jq -r ".name"'
   when = 'pm status --quiet'
   format = "📁 [$output](bold blue) "
   ```

### Team Sharing

팀에서 일관된 설정을 사용하는 방법:

```bash
# 1. 팀 공용 설정 파일 생성
pm starship --style basic --show > team-starship-config.toml

# 2. 팀 저장소에 추가
git add team-starship-config.toml
git commit -m "Add team PM Starship configuration"

# 3. 팀원들이 사용
cat team-starship-config.toml >> ~/.config/starship.toml
```

### Maintenance

정기적인 유지보수:

```bash
# 1. 설정 테스트
pm starship --test

# 2. 캐시 정리
rm -f /tmp/pm_status_*

# 3. 설정 업데이트
pm starship --show > /tmp/new-config.toml
diff ~/.config/starship.toml /tmp/new-config.toml
```

## 🔗 Related Documentation

- [STARSHIP_INTEGRATION.md](STARSHIP_INTEGRATION.md) - 상세한 Starship 연동 가이드
- [COMMANDS.md](COMMANDS.md) - 전체 명령어 레퍼런스
- [README.md](../README.md) - PM 프로젝트 개요
- [Starship 공식 문서](https://starship.rs/config/) - Starship 설정 가이드

## 🆘 Getting Help

문제가 있거나 도움이 필요한 경우:

1. **GitHub Issues**: [https://github.com/zdpk/project-manager/issues](https://github.com/zdpk/project-manager/issues)
2. **Documentation**: 이 문서와 관련 문서들
3. **Command Help**: `pm --help`, `pm starship --help`
4. **Community**: GitHub Discussions 또는 Issues

---

🎉 이제 PM과 Starship을 완벽하게 연동하여 터미널에서 프로젝트 정보를 확인할 수 있습니다!