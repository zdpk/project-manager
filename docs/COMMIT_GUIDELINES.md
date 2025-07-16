# 커밋 가이드라인

## 원칙

### 1. 논리적 단위로 커밋 분리
- 각 커밋은 하나의 논리적 변경사항 또는 기능을 포함
- 관련된 코드, 문서, 테스트는 동일한 커밋에 포함
- 큰 기능은 여러 개의 작은 논리적 단위로 분할

### 2. 코드/문서/테스트 일체화
- 기능 구현 + 관련 문서 업데이트 + 테스트 코드 = 하나의 커밋
- 문서만 변경하는 경우는 예외적으로 독립적 커밋 가능
- 테스트는 해당 기능과 함께 커밋

### 3. 순차적 의존성 고려
- 각 커밋은 이전 커밋에 의존해야 함
- 독립적으로 동작 가능한 상태여야 함
- 빌드 및 기본 테스트 통과 필수

## 커밋 메시지 형식

```
<type>: <description>

<body explaining what and why>

<optional footer with breaking changes, issues, etc.>
```

### Type 분류
- `feat`: 새로운 기능 추가
- `fix`: 버그 수정
- `refactor`: 코드 리팩토링 (기능 변경 없음)
- `docs`: 문서 변경
- `test`: 테스트 코드 추가/수정
- `chore`: 빌드 시스템, 의존성 등
- `perf`: 성능 개선
- `style`: 코드 스타일 변경 (포맷팅 등)

### 예시

#### 좋은 커밋 메시지
```
feat: Add interactive tag selection to add command

- Implement multi-select tag interface with existing tags
- Show tag usage counts for better UX
- Add create-new-tag functionality
- Update add command documentation
- Add tests for tag selection logic

This improves user experience by making tag management more intuitive
and reduces typing for commonly used tags.
```

#### 나쁜 커밋 메시지
```
fix stuff

- fixed some bugs
- updated docs
- misc changes
```

## 커밋 계획 수립

### 1. 작업 시작 전 계획 수립
```markdown
# Feature X 구현 계획

## 목표
- 주요 목표와 기대 효과

## 커밋 계획
### Commit 1: <type>: <description>
- 변경할 파일 목록
- 주요 변경사항
- 포함할 테스트
- 관련 문서 업데이트

### Commit 2: <type>: <description>
- ...
```

### 2. 계획 저장
- `.plan.md` 파일에 상세 계획 저장
- 각 커밋의 목적과 범위 명시
- 의존성 관계 정의

### 3. 계획 추적
- 각 커밋 완료 후 체크리스트 업데이트
- 계획 변경시 문서 업데이트
- 최종 검토 및 정리

## 실행 프로세스

### 1. 커밋 전 체크리스트
- [ ] 빌드 성공 (`cargo build`)
- [ ] 기본 테스트 통과 (`cargo test`)
- [ ] 관련 문서 업데이트 완료
- [ ] 커밋 메시지 검토 완료

### 2. 커밋 실행
```bash
# 변경사항 확인
git status
git diff

# 스테이징
git add <files>

# 커밋
git commit -m "type: description" -m "detailed body"
```

### 3. 커밋 후 확인
- [ ] 커밋 히스토리 확인
- [ ] 다음 커밋 계획 점검
- [ ] 빌드 및 테스트 재확인

## 브랜치 전략

### 1. 기능별 브랜치
- `feat/feature-name`: 새로운 기능
- `fix/bug-description`: 버그 수정  
- `refactor/component-name`: 리팩토링
- `docs/section-name`: 문서 업데이트

### 2. 브랜치 명명 규칙
- 소문자 사용
- 하이픈으로 단어 구분
- 구체적이고 명확한 이름

### 3. PR 준비
- 각 브랜치는 논리적으로 완결된 기능 단위
- 리뷰하기 쉬운 크기로 유지
- 관련 문서 포함

## 코드 리뷰 고려사항

### 1. 리뷰어 관점
- 각 커밋의 목적이 명확한가?
- 변경사항이 논리적으로 일관된가?
- 문서와 코드가 일치하는가?
- 테스트가 충분한가?

### 2. 작성자 관점
- 커밋 메시지가 변경사항을 잘 설명하는가?
- 각 커밋이 독립적으로 검토 가능한가?
- 관련 문서가 함께 업데이트되었는가?

## 예외 상황 처리

### 1. 긴급 수정
- 핫픽스는 별도 브랜치에서 진행
- 가능한 한 빨리 문서 업데이트
- 사후 정리 커밋으로 표준화

### 2. 대규모 리팩토링
- 단계별 계획 수립 필수
- 각 단계가 독립적으로 동작해야 함
- 중간 단계에서도 안정성 확보

### 3. 의존성 변경
- 의존성 변경은 별도 커밋으로 분리
- 영향 범위 명시
- 마이그레이션 가이드 포함

## 도구 및 자동화

### 1. 커밋 메시지 템플릿
```bash
# .gitmessage 파일 설정
git config commit.template .gitmessage
```

### 2. 커밋 훅
- pre-commit: 빌드 및 테스트 실행
- commit-msg: 커밋 메시지 형식 검증

### 3. 자동화 스크립트
- 커밋 전 체크리스트 자동 실행
- 브랜치 생성 및 초기 설정 자동화

## 모범 사례

### 1. 작은 커밋 지향
- 하나의 변경사항에 집중
- 리뷰하기 쉬운 크기 유지
- 롤백 시 영향 최소화

### 2. 의미 있는 커밋 메시지
- 왜 변경했는지 설명
- 어떤 문제를 해결했는지 명시
- 향후 유지보수자를 위한 컨텍스트 제공

### 3. 일관성 유지
- 팀 내에서 동일한 형식 사용
- 정기적인 가이드라인 리뷰
- 지속적인 개선

이 가이드라인을 따르면 코드 히스토리가 명확해지고, 리뷰 과정이 효율적이 되며, 장기적인 유지보수가 용이해집니다.