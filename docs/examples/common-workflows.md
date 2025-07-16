# Common Tag Selection Workflows

This document showcases real-world usage patterns and workflows for the two-step tag selection interface.

## Workflow 1: Frontend Developer Daily Routine

### Morning: Quick Script Creation
```bash
$ pm add ./debug-helper

? What would you like to do?
  > Create Project [debug-helper] (without tags)
    Add tags to this project
    Create new tag and add to project

✅ Successfully added project 'debug-helper'
   Path: /Users/dev/projects/debug-helper
```
**Rationale**: Temporary debugging scripts don't need tags.

### Mid-day: New React Component Library
```bash
$ pm add ./ui-components

? What would you like to do?
    Create Project [ui-components] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] frontend (8 projects)
  [ ] react (5 projects)
  [ ] typescript (6 projects)
  [ ] components (2 projects)
  [ ] storybook (1 projects)

# Type "react" to filter:
🏷️ Select tags for this project (type to filter): react
  [x] react (5 projects)

# Clear filter, type "comp":
🏷️ Select tags for this project (type to filter): comp  
  [x] react (5 projects)
  [x] components (2 projects)

✅ Successfully added project 'ui-components' with tags: react, components
   Path: /Users/dev/projects/ui-components
```
**Rationale**: Leverage existing tag taxonomy for consistent categorization.

### Evening: Personal Side Project
```bash
$ pm add ./crypto-tracker

? What would you like to do?
    Create Project [crypto-tracker] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: personal
? Add another new tag? Yes

✨ Create new tag: crypto
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [x] react (5 projects)
  [x] typescript (6 projects)

✅ Successfully added project 'crypto-tracker' with tags: personal, crypto, react, typescript
   Path: /Users/dev/projects/crypto-tracker
```
**Rationale**: New project category (crypto) + personal classification + existing tech stack.

---

## Workflow 2: Full-Stack Developer Project Organization

### Backend Microservice
```bash
$ pm add ./user-service

? What would you like to do?
    Create Project [user-service] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: microservice
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter): back
  [x] backend (4 projects)

# Clear filter, type "node":
🏷️ Select tags for this project (type to filter): node
  [x] backend (4 projects)
  [x] nodejs (3 projects)

# Clear filter, type "api":
🏷️ Select tags for this project (type to filter): api
  [x] backend (4 projects)
  [x] nodejs (3 projects)
  [x] api (2 projects)

✅ Successfully added project 'user-service' with tags: microservice, backend, nodejs, api
   Path: /Users/dev/projects/user-service
```

### Frontend SPA
```bash
$ pm add ./admin-dashboard

? What would you like to do?
    Create Project [admin-dashboard] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] frontend (9 projects)
  [ ] react (6 projects)
  [ ] typescript (7 projects)
  [ ] components (2 projects)
  [ ] storybook (1 projects)
  [ ] backend (4 projects)
  [ ] nodejs (3 projects)
  [ ] api (3 projects)
  [ ] microservice (1 projects)
  [ ] personal (1 projects)
  [ ] crypto (1 projects)

# Multi-select workflow:
🏷️ Select tags for this project (type to filter):
  [x] frontend (9 projects)
  [x] react (6 projects)
  [x] typescript (7 projects)

✅ Successfully added project 'admin-dashboard' with tags: frontend, react, typescript
   Path: /Users/dev/projects/admin-dashboard
```

---

## Workflow 3: Data Scientist Project Setup

### Research Project
```bash
$ pm add ./sentiment-analysis

? What would you like to do?
    Create Project [sentiment-analysis] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: nlp
? Add another new tag? Yes

✨ Create new tag: research
? Add another new tag? Yes

✨ Create new tag: sentiment-analysis
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter): python
  [x] python (4 projects)

# Clear filter, type "machine":
🏷️ Select tags for this project (type to filter): machine
  [x] python (4 projects)
  [x] machine-learning (2 projects)

✅ Successfully added project 'sentiment-analysis' with tags: nlp, research, sentiment-analysis, python, machine-learning
   Path: /Users/scientist/projects/sentiment-analysis
```

### Data Pipeline
```bash
$ pm add ./etl-pipeline

? What would you like to do?
    Create Project [etl-pipeline] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] python (5 projects)
  [ ] machine-learning (2 projects)
  [ ] nlp (1 projects)
  [ ] research (1 projects)
  [ ] sentiment-analysis (1 projects)

# Type "data" to filter (but no existing "data" tags):
🏷️ Select tags for this project (type to filter): data
  # No matches

# Clear and select python:
🏷️ Select tags for this project (type to filter):
  [x] python (5 projects)

# User realizes they need new tags, cancels and restarts:
^C

$ pm add ./etl-pipeline

? What would you like to do?
    Create Project [etl-pipeline] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: etl
? Add another new tag? Yes

✨ Create new tag: data-pipeline
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [x] python (5 projects)

✅ Successfully added project 'etl-pipeline' with tags: etl, data-pipeline, python
   Path: /Users/scientist/projects/etl-pipeline
```
**Learning**: User discovered they needed new category tags, restarted process.

---

## Workflow 4: DevOps Engineer Infrastructure Projects

### Kubernetes Setup
```bash
$ pm add ./k8s-cluster-config

? What would you like to do?
    Create Project [k8s-cluster-config] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: kubernetes
? Add another new tag? Yes

✨ Create new tag: infrastructure
? Add another new tag? Yes

✨ Create new tag: devops
? Add another new tag? No

? Add existing tags as well? No

✅ Successfully added project 'k8s-cluster-config' with tags: kubernetes, infrastructure, devops
   Path: /Users/devops/projects/k8s-cluster-config
```

### Monitoring Stack
```bash
$ pm add ./prometheus-grafana

? What would you like to do?
    Create Project [prometheus-grafana] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] kubernetes (1 projects)
  [ ] infrastructure (1 projects)
  [ ] devops (1 projects)

# Select existing infrastructure tags:
🏷️ Select tags for this project (type to filter):
  [x] infrastructure (1 projects)
  [x] devops (1 projects)
  [x] kubernetes (1 projects)

✅ Successfully added project 'prometheus-grafana' with tags: infrastructure, devops, kubernetes
   Path: /Users/devops/projects/prometheus-grafana
```

---

## Workflow 5: Learning and Experimentation

### Tutorial Following
```bash
$ pm add ./react-tutorial

? What would you like to do?
    Create Project [react-tutorial] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: tutorial
? Add another new tag? Yes

✨ Create new tag: learning
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter): react
  [x] react (6 projects)

✅ Successfully added project 'react-tutorial' with tags: tutorial, learning, react
   Path: /Users/student/projects/react-tutorial
```

### Code Challenge
```bash
$ pm add ./leetcode-solutions

? What would you like to do?
    Create Project [leetcode-solutions] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: algorithms
? Add another new tag? Yes

✨ Create new tag: leetcode
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter): python
  [x] python (5 projects)

# Clear filter, type "learn":
🏷️ Select tags for this project (type to filter): learn
  [x] python (5 projects)
  [x] learning (1 projects)

✅ Successfully added project 'leetcode-solutions' with tags: algorithms, leetcode, python, learning
   Path: /Users/student/projects/leetcode-solutions
```

---

## Workflow 6: Team Project Organization

### Client Work
```bash
$ pm add ./acme-website

? What would you like to do?
    Create Project [acme-website] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: client-acme
? Add another new tag? Yes

✨ Create new tag: billable
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter): front
  [x] frontend (9 projects)

# Clear filter, type "react":
🏷️ Select tags for this project (type to filter): react
  [x] frontend (9 projects)
  [x] react (6 projects)

✅ Successfully added project 'acme-website' with tags: client-acme, billable, frontend, react
   Path: /Users/freelancer/projects/acme-website
```

### Internal Tool
```bash
$ pm add ./team-dashboard

? What would you like to do?
    Create Project [team-dashboard] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: internal
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [x] frontend (10 projects)
  [x] react (7 projects)
  [x] typescript (7 projects)

✅ Successfully added project 'team-dashboard' with tags: internal, frontend, react, typescript
   Path: /Users/freelancer/projects/team-dashboard
```

---

## Tag Evolution Patterns

### Week 1: Basic Tags
```
frontend (3 projects)
backend (2 projects)
python (2 projects)
react (2 projects)
```

### Week 4: Category Refinement
```
frontend (8 projects)
backend (6 projects)
python (5 projects)
react (6 projects)
typescript (4 projects)
api (4 projects)
personal (3 projects)
learning (2 projects)
```

### Month 3: Specialized Classification
```
frontend (15 projects)
backend (12 projects)
python (8 projects)
react (10 projects)
typescript (8 projects)
api (7 projects)
personal (8 projects)
learning (5 projects)
client-acme (3 projects)
client-beta (2 projects)
microservice (4 projects)
kubernetes (3 projects)
infrastructure (3 projects)
devops (3 projects)
machine-learning (3 projects)
nlp (2 projects)
tutorial (4 projects)
```

**Pattern**: Tags evolve from general to specific as project portfolio grows.

---

## Efficiency Tips Learned from Usage

### 1. Consistent Tag Vocabulary
- Establish naming conventions early
- Use lowercase-with-hyphens: `machine-learning`, `client-acme`
- Be specific but not overly verbose

### 2. Filtering Strategies
- Use unique prefixes: "ml" for machine-learning tags
- Common filters: "client", "personal", "learning", "api"
- Start broad, then narrow down

### 3. Workflow Selection
- **"No tags"**: For temporary/experimental projects
- **"Add existing"**: For projects fitting established categories  
- **"Create new"**: For new domains or specific client work

### 4. Tag Maintenance
```bash
# Regular tag analysis
pm tag list

# Renaming through tag commands
pm tag remove project-name old-tag
pm tag add project-name new-tag
```

### 5. Batch Operations
```bash
# Skip interactive selection for batch adds
pm add *

# Add tags afterward using tag commands
pm tag add project1 frontend react
pm tag add project2 backend api
```

These workflows demonstrate the flexibility and power of the new two-step tag selection interface across different user types and project scenarios.