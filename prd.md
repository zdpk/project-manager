# PM (Project Manager) - 상세 제품 요구사항 문서 v1.0

##  **프로젝트 배경 및 목적**

### **문제 정의**
현대 개발자들은 동시에 여러 프로젝트를 관리해야 합니다. 특히 마이크로서비스 아키텍처, 프론트엔드/백엔드 분리, 개인 프로젝트 등으로 인해 하나의 개발자가 5-20개의 독립적인 repository를 오가며 작업하는 것이 일반적입니다.

**주요 문제점들:**
1. **컨텍스트 스위칭 비용**: 프로젝트 간 전환 시 "이게 뭐하는 프로젝트였지?" 순간
2. **경로 기억 부담**: `cd ~/workspace/company/backend/user-service/` 같은 긴 경로 기억
3. **분산된 정보**: 조직 repo, 개인 repo, 다양한 Git 호스팅 서비스에 분산
4. **일관성 없는 워크플로우**: 프로젝트마다 다른 실행 방법, 다른 도구들
5. **시간 추적 어려움**: 어떤 프로젝트에 얼마나 시간을 투자했는지 모름

### **타겟 사용자**
**Primary Persona: "멀티프로젝트 개발자"**
- Helix 에디터 + Claude Code를 주로 사용
- 터미널 중심의 워크플로우 선호
- 5-20개 프로젝트 동시 관리
- 효율성과 생산성을 중시
- 도구 설정에 시간 쓰기보다 개발에 집중하고 싶어함

### **핵심 가치 제안**
PM은 **"프로젝트 탐색과 전환을 제로에 가깝게 만드는"** 도구입니다. 개발자가 프로젝트를 찾고, 열고, 컨텍스트를 로드하는 데 드는 정신적 에너지를 최소화하여, 실제 개발과 문제 해결에 집중할 수 있게 합니다.

##  **핵심 개념 및 설계 철학**

### **1. 제로 프릭션 원칙**
모든 명령어는 3초 이내에 완료되어야 하고, 사용자가 "다음에 뭘 해야 하지?"라고 고민할 시간을 주지 않아야 합니다.

**예시:**
```bash
pm s frontend-app
# 실행 즉시:
# 1. 디렉토리 이동 (< 10ms)
# 2. Helix 에디터 실행 (< 500ms)  
# 3. 최근 파일들 자동 로드
# 4. Git 상태 표시
# → 총 1초 내에 작업 가능한 상태
```

### **2. 스마트 기본값 원칙**
사용자가 설정하지 않아도 합리적으로 동작해야 합니다. 설정은 필요에 의해서만 제공합니다.

**예시:**
- 프로젝트 이름: 지정하지 않으면 디렉토리명 사용
- 태그: Git URL에서 자동으로 조직/언어 추론
- 에디터: Helix를 기본값으로, 환경변수로 오버라이드 가능

### **3. 충돌 없는 동기화**
여러 머신에서 사용해도 데이터 충돌이 발생하지 않도록 설계합니다. 각 머신의 고유한 정보(접근 시간, 횟수)는 분리하고, 공통 정보(프로젝트 정의)만 공유합니다.

### **4. 점진적 공개**
처음 사용자는 `pm add`, `pm ls`, `pm s`만 알아도 충분하고, 필요에 따라 고급 기능을 학습할 수 있도록 합니다.

##  **상세 사용자 시나리오**

### **`pm init` - 초기 설정**

**목적**: `pm` 도구를 처음 사용할 때 필요한 전역 설정을 초기화합니다.

**동작 과정**:
1.  **설정 파일 확인**: `~/.config/pm/config.json` 파일의 존재 여부를 확인합니다.
2.  **중복 실행 방지**: 만약 파일이 이미 존재한다면, "이미 초기화되었습니다." 메시지를 출력하고 실행을 중단합니다.
3.  **사용자 정보 입력**: 파일이 없을 경우, 다음 두 가지 정보를 사용자에게 입력받습니다.
    *   GitHub 사용자 이름 (예: `zdpk`)
    *   프로젝트 루트 디렉토리의 절대 경로 (예: `~/workspace`)
4.  **설정 파일 생성**: 입력받은 정보와 기본값을 사용하여 `config.json` 파일을 생성합니다.

**결과 예시**:
```bash
$ pm init
GitHub username: zdpk
Projects root directory path: ~/workspace
✅ pm이 성공적으로 초기화되었습니다.
   설정 파일: ~/.config/pm/config.json
```

### **`pm add` - 프로젝트 등록**

**목적**: 새로운 프로젝트를 PM의 관리 대상에 추가합니다.

**스마트 기능**:
*   **경로 자동 완성**: `pm init` 시 설정한 `projects_root_dir`를 기반으로 경로를 해석합니다.
    *   `pm add my-project` → `~/workspace/my-project`로 자동 변환
    *   `pm add company/frontend` → `~/workspace/company/frontend`로 자동 변환
    *   절대 경로(`pm add /path/to/another/project`)도 그대로 지원됩니다.
*   이름 생략시 디렉토리명 자동 사용
*   Git remote URL에서 조직명/프로젝트명 추론
*   주요 언어 자동 감지하여 태그 추가 제안

---

(기존 `pm add` 설명의 나머지 부분은 동일)

---

### **데이터 모델 설계 의도**

(기존 설명에 추가)

### **Config 구조체**

**`github_username`**: `gh` CLI나 Git 설정에서 자동으로 가져오는 대신, 사용자가 명시적으로 입력하도록 하여 여러 GitHub 계정을 사용하는 경우의 모호함을 없앱니다.

**`projects_root_dir`**: `pm add` 명령어 사용 시 편의성을 극대화하고, 사용자가 일관된 디렉토리 구조를 유지하도록 유도합니다.

# 작업 시작
pm s frontend
# → frontend 프로젝트로 이동, helix 실행
# → 하루종일 frontend 작업

# 오후에 긴급 버그 수정
pm s api-gateway  
# → 즉시 context switch, 이전 작업 자동 저장
# → api-gateway 프로젝트 환경 로드
```

### **시나리오 2: 경험 많은 개발자의 복잡한 워크플로우**
```bash
# 아침에 할 일 확인
pm ls --tags work --recent 3d
# → 회사 프로젝트 중 최근 3일간 업데이트된 것들만 표시
# → Git 커밋, 이슈, PR 상태까지 한 눈에 파악

# 긴급도 높은 프로젝트부터 처리
pm ls --tags urgent --limit 3
# → 긴급 태그가 달린 프로젝트 상위 3개

# 각 프로젝트마다 짧은 작업
pm s payment-service    # 결제 서비스 버그 수정 (30분)
pm s user-dashboard     # UI 개선 (1시간)  
pm s api-docs          # 문서 업데이트 (15분)

# 개인 프로젝트 시간
pm ls --tags personal
pm s weekend-project
```

### **시나리오 3: 팀 협업 상황**
```bash
# 팀원이 "user-service에서 이상한 일이 일어나고 있어"라고 말함
pm s user-service
# → 즉시 해당 프로젝트로 이동
# → 최근 변경사항, Git 로그, 실행 중인 프로세스 확인 가능

# 관련 프로젝트들도 확인 필요
pm ls --tags microservice,user
# → user-service와 관련된 모든 마이크로서비스 표시
# → 의존성 관계 파악 가능
```

## ️ **명령어별 상세 설명**

### **`pm add` - 프로젝트 등록**

**목적**: 새로운 프로젝트를 PM의 관리 대상에 추가합니다.

**동작 과정**:
1. 경로 검증 및 정규화 (상대경로 → 절대경로)
2. 중복 확인 (같은 경로의 프로젝트가 이미 있는지)
3. Git 정보 자동 감지 (repository 여부, remote URL, 주 브랜치)
4. 프로그래밍 언어 자동 감지 (파일 확장자 기반)
5. 고유 ID 생성 (UUID v4)
6. 메타데이터와 함께 저장

**스마트 기능**:
- 이름 생략시 디렉토리명 자동 사용
- Git remote URL에서 조직명/프로젝트명 추론
- 주요 언어 자동 감지하여 태그 추가 제안

```bash
# 기본 사용
pm add ~/workspace/my-app

# 상세 정보와 함께
pm add ~/workspace/frontend --tags "frontend,react,work" --description "고객용 웹 애플리케이션"

# 결과 예시:
# ✅ 프로젝트가 추가되었습니다: frontend
#    경로: /home/user/workspace/frontend  
#    태그: frontend, react, work
#    언어: JavaScript (자동 감지)
#    Git: https://github.com/company/frontend.git
```

### **`pm ls` - 프로젝트 목록**

**목적**: 등록된 프로젝트들을 다양한 조건으로 필터링하여 표시합니다.

**정렬 우선순위**:
1. Git 마지막 커밋 시간 (실제 개발 활동 기준)
2. 프로젝트 정보 마지막 수정 시간
3. 생성 시간

**표시 정보**:
- 프로젝트명 (색상 코딩)
- 주요 태그 (최대 3개)
- 마지막 업데이트 시간 (상대시간, "2시간 전")
- Git 상태 (옵션)

**필터링 로직**:
```bash
# AND 조건 (모든 태그를 포함하는 프로젝트)
pm ls --tags frontend,react
# → frontend AND react 태그를 모두 가진 프로젝트

# OR 조건 (태그 중 하나라도 포함하는 프로젝트)  
pm ls --tags-any frontend,backend
# → frontend OR backend 태그를 가진 프로젝트

# 시간 기반 필터링
pm ls --recent 7
# → 최근 7일 내에 Git 커밋이나 파일 수정이 있었던 프로젝트

# 결과 제한
pm ls --limit 5
# → 상위 5개 프로젝트만 표시
```

**출력 예시**:
```
 Active Projects (8 found)

 frontend-app          [frontend, react]           2시간 전
 api-gateway          [backend, rust]             어제  
 mobile-app           [frontend, flutter]         3일 전
 analytics-service    [backend, python]           1주일 전
️  deployment-scripts   [devops, bash]              2주일 전
```

### **`pm s` - 프로젝트 전환**

**목적**: 지정된 프로젝트로 즉시 전환하여 작업 가능한 상태로 만듭니다.

**동작 과정**:
1. 프로젝트 식별 (이름 또는 ID로)
2. 접근 통계 업데이트 (마지막 접근 시간, 접근 횟수)
3. 현재 디렉토리 변경
4. 에디터 실행 (기본: Helix)
5. 프로젝트 상태 표시

**접근 통계의 목적**:
- `pm ls` 시 자주 사용하는 프로젝트가 상위에 표시
- `pm sg` (추천) 기능에서 사용 패턴 분석
- 향후 시간 추적 기능의 기반

**에디터 통합**:
- 기본적으로 Helix 에디터 실행
- `--no-editor` 옵션으로 에디터 실행 생략 가능
- 향후 최근 작업 파일 자동 로드 기능 계획

```bash
pm s frontend-app
# 출력:
#  frontend-app 프로젝트로 전환
#  helix로 프로젝트를 열었습니다
#  Git: main 브랜치, 2 commits ahead
# ⏰ 마지막 작업: 2시간 전

pm s api-gateway --no-editor  
# 출력:
#  api-gateway 프로젝트로 전환
#  /home/user/workspace/api-gateway
```

### **`pm tag` - 태그 관리**

**목적**: 프로젝트들을 논리적으로 그룹화하고 분류하기 위한 태그 시스템을 관리합니다.

**태그 정규화 규칙**:
- 소문자로 변환
- 공백은 하이픈(-)으로 변경
- 특수문자 제거 (하이픈, 언더스코어만 허용)
- 중복 제거 및 정렬

**사용 패턴**:
```bash
# 조직별 분류
pm tag add project-a work,company-internal
pm tag add side-project personal,opensource

# 기술별 분류  
pm tag add frontend-app frontend,react,typescript
pm tag add api-server backend,rust,microservice

# 상태별 분류
pm tag add legacy-system maintenance,deprecated
pm tag add new-feature active,high-priority

# 태그 통계 확인
pm tag all
# 출력:
#  모든 태그 (사용 빈도순):
#    work (12 projects)
#    frontend (8 projects)  
#    backend (6 projects)
#    personal (4 projects)
#    urgent (2 projects)
```

### **`pm sg` - 스마트 추천** (향후 구현)

**목적**: 현재 상황과 컨텍스트를 고려하여 작업할 프로젝트를 추천합니다.

**추천 알고리즘 요소**:
1. 최근 접근 패턴
2. 시간대별 작업 패턴 (아침엔 집중 필요한 작업, 오후엔 간단한 작업)
3. 에너지 레벨에 따른 작업 복잡도
4. 프로젝트 간 의존성
5. 마감일 정보 (태그 기반)

##  **데이터 모델 설계 의도**

### **Project 구조체**

**ID 필드**: UUID v4를 사용하는 이유
- 프로젝트명은 변경될 수 있지만 ID는 불변
- 여러 머신 간 동기화 시 고유성 보장
- 향후 의존성 관계 표현 시 안정적인 참조

**경로 필드**: 절대경로를 저장하는 이유
- 상대경로는 실행 위치에 따라 변할 수 있음
- 심볼릭 링크 해결을 통한 일관성 보장
- 크로스 플랫폼 호환성

**태그 배열**: 단순 문자열 배열을 사용하는 이유
- 복잡한 태그 계층 구조보다 플랫 구조가 사용하기 쉬움
- 필터링 로직이 단순해짐
- JSON 직렬화가 간단함

**세 개의 시간 필드**:
- `created_at`: 프로젝트 등록 시점 (변경 불가)
- `updated_at`: 프로젝트 정보 수정 시점 (태그 변경 등)
- `git_updated_at`: 실제 개발 활동 시점 (Git 커밋 기준)

### **MachineMetadata 분리 설계**

**충돌 방지**: 여러 머신에서 동시 사용 시 데이터 충돌 최소화
```json
{
  "projects": {
    "frontend-app": { /* 모든 머신이 공유하는 정보 */ }
  },
  "machine_metadata": {
    "laptop-home": {
      "last_accessed": { "frontend-app": "2025-01-10T15:30:00Z" },
      "access_counts": { "frontend-app": 45 }
    },
    "desktop-office": {
      "last_accessed": { "frontend-app": "2025-01-10T14:20:00Z" },
      "access_counts": { "frontend-app": 23 }
    }
  }
}
```

이 구조를 통해:
- 프로젝트 정의는 모든 머신에서 동일
- 접근 통계는 머신별로 독립적
- 전체 통계는 모든 머신의 데이터를 합산하여 계산

##  **워크플로우 최적화**

### **Cold Start 최소화**
프로그램 시작부터 사용 가능한 상태까지의 시간을 최소화합니다:

1. **설정 로딩**: JSON 파일 하나만 읽기 (< 10ms)
2. **Git 상태 확인**: 백그라운드에서 비동기 처리
3. **에디터 실행**: 논블로킹 프로세스 spawn

### **Hot Path 최적화**
가장 자주 사용되는 명령어들의 성능을 우선 최적화합니다:

1. `pm s project-name` (가장 빈번)
2. `pm ls` (두 번째로 빈번)
3. `pm ls --tags work` (필터링된 목록)

### **점진적 정보 로드**
필요한 정보만 순차적으로 로드하여 초기 응답성을 높입니다:

```
pm ls 실행 시:
1. 즉시: 기본 프로젝트 정보 표시 (이름, 태그, 기본 시간)
2. 백그라운드: Git 상태 확인
3. 백그라운드: 언어 감지 재확인
4. 필요시: 상세 정보 업데이트
```

##  **사용자 경험 설계**

### **출력 형식 철학**

**일관된 색상 코딩**:
-  (빨강): 긴급하거나 최근 활동
-  (파랑): 백엔드/시스템 프로젝트  
-  (초록): 프론트엔드/UI 프로젝트
-  (노랑): 데이터/분석 프로젝트
- ️ (회색): 도구/인프라 프로젝트

**상대 시간 표시**:
절대 시간보다 상대 시간이 인지적으로 처리하기 쉽습니다:
- "2시간 전", "어제", "3일 전", "1주일 전"
- 24시간 이내: 시간 단위
- 7일 이내: 일 단위  
- 그 이후: 주/월 단위

**프로그레시브 정보 공개**:
```bash
# 기본 목록 - 간결한 정보
pm ls
#  frontend-app    [react]    2시간 전

# 상세 목록 - 더 많은 정보
pm ls --detailed
#  frontend-app
#    태그: frontend, react, typescript, work
#    경로: ~/workspace/company/frontend
#    언어: JavaScript (TypeScript 95%)
#    Git: main 브랜치, 2 commits ahead, 1 file changed
#    마지막 작업: 2시간 전 (login-form.tsx 수정)
```

### **에러 처리 전략**

**친근한 에러 메시지**: 기술적 세부사항보다 사용자가 취해야 할 행동에 집중
```bash
# 나쁜 예
pm s nonexistent
# Error: Project with identifier 'nonexistent' not found in database

# 좋은 예  
pm s nonexistent
# ❌ 프로젝트를 찾을 수 없습니다: nonexistent
# 
#  비슷한 프로젝트들:
#    - frontend-app
#    - backend-app
#
#  새 프로젝트를 등록하려면: pm add <경로>
```

**자동 복구 제안**: 사용자가 실수했을 때 대안을 제시
```bash
pm add ~/nonexistent/path
# ❌ 경로가 존재하지 않습니다: ~/nonexistent/path
#
#  비슷한 경로들:
#    - ~/workspace/path
#    - ~/projects/path
#
#  디렉토리를 먼저 생성하려면: mkdir -p ~/nonexistent/path
```

##  **성능 요구사항**

### **응답 시간 목표**
- `pm s project`: < 1초 (에디터 실행 포함)
- `pm ls`: < 200ms (기본 목록)
- `pm ls --detailed`: < 500ms (Git 상태 포함)
- `pm add`: < 300ms (Git 분석 포함)

### **메모리 사용량**
- 기본 실행: < 10MB
- 100개 프로젝트 로드: < 50MB
- Git 상태 캐싱: < 100MB

### **파일 I/O 최적화**
- 설정 파일 읽기: mmap 사용 고려
- Git 저장소 분석: 병렬 처리
- 대용량 디렉토리 스캔: 조기 종료 최적화

##  **기술적 고려사항**

### **크로스 플랫폼 호환성**
- 경로 구분자: `std::path::Path` 사용
- 홈 디렉토리: `dirs` crate 사용
- 프로세스 실행: `tokio::process` 사용

### **에러 복구**
- 설정 파일 손상: 자동 백업에서 복구
- Git 저장소 문제: 경고 표시 후 계속 진행
- 권한 문제: 대안 경로 제안

### **보안 고려사항**
- 설정 파일 권한: 사용자만 읽기/쓰기 가능
- 경로 검증: 디렉토리 트래버설 공격 방지
- 명령어 실행: 셸 인젝션 방지

### **확장성 설계**
- 플러그인 시스템: 향후 확장을 위한 훅 포인트
- 설정 스키마: 버전 관리 및 마이그레이션
- API 설계: 다른 도구와의 통합 가능성

이러한 상세한 설명을 바탕으로 Claude Code가 사용자의 의도를 정확히 이해하고 구현할 수 있을 것입니다!
