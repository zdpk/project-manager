# GEMINI Agent Instructions

> **Purpose**: Guide for AI agents working on this project  
> **Last Updated**: 2025-07-29  
> **Version**: 1.0

## 📋 Project Overview

This is a Rust CLI project manager (`pm`) with automated NPM distribution. The project uses GitHub Actions for CI/CD and publishes both GitHub Releases and NPM packages.

## 📁 Plan File System

### Plan File Location
All planning documents are stored in: `docs/plans/`

### Plan File Types
- **Implementation Plans**: Step-by-step technical implementation guides
- **Architecture Plans**: System design and structural changes
- **Deployment Plans**: Release and distribution strategies

### How to Handle Plan Files

#### 1. Reading Plan Files
When given a task, ALWAYS check for existing plan files first:

```bash
# Check for relevant plan files
ls docs/plans/
```

**Priority Order:**
1. Look for specific plan files mentioned by the user
2. Check for related plans (e.g., DEPLOYMENT_* for deployment tasks)
3. Read the most recent plan files (check timestamps)

#### 2. Following Plan Structure
Plan files use this standard structure:
- **📋 개요**: Task overview and context
- **🚨 현재 문제점**: Current issues to solve
- **🎯 해결 방안**: Solution approaches by phase
- **🔧 구현 단계**: Detailed implementation steps
- **✅ 성공 지표**: Success criteria
- **📝 작업 체크리스트**: Actionable tasks
- **🚀 Agent 실행 지침**: Specific instructions for agents

#### 3. Plan Execution Guidelines

**MUST DO:**
- Read the entire plan file before starting work
- Follow the priority order (Critical → High → Medium → Low)
- Update the checklist as you complete tasks
- Reference specific file paths and code snippets mentioned in plans

**SHOULD DO:**
- Update plan files if you discover new issues or solutions
- Add progress notes to the plan file
- Cross-reference with other relevant documentation

**NEVER DO:**
- Skip reading the plan file completely
- Work out of priority order without justification
- Ignore the success criteria
- Modify core plan structure without documentation

#### 4. Plan File Updates

When updating plan files:
```markdown
## 📝 Progress Log

### 2025-07-29 15:30 - Agent Update
- ✅ Completed: NPM script modification
- 🔄 In Progress: Workflow testing
- ❌ Blocked: Need user input on version strategy
- 📝 Notes: Found additional issue with TypeScript compilation
```

#### 5. Creating New Plan Files

If creating new plan files, follow this naming convention:
- `[CATEGORY]_[SUBJECT]_PLAN.md`
- Examples: `DEPLOYMENT_AUTOMATION_PLAN.md`, `ARCHITECTURE_REFACTOR_PLAN.md`

Use this template:
```markdown
# [Title] Plan

> **Generated**: [Date]  
> **Status**: [Draft|Ready|In Progress|Completed]  
> **Priority**: [Critical|High|Medium|Low]  

## 📋 개요
[Brief description]

## 🚨 현재 문제점
[Issues to solve]

## 🎯 해결 방안
[Solutions by phase]

## 🔧 구현 단계
[Step-by-step implementation]

## ✅ 성공 지표
[Success criteria]

## 📝 작업 체크리스트
[Actionable items]

## 🚀 Agent 실행 지침
[Specific instructions]
```

## 🔧 Current Active Plans

### Deployment Automation
**File**: `docs/plans/DEPLOYMENT_AUTOMATION_PLAN.md`  
**Status**: Ready for Implementation  
**Priority**: High  
**Context**: Fix NPM deployment failures in GitHub Actions workflow

## 🎯 Agent Workflow

### Step 1: Plan Discovery
```bash
# Always start with this
find docs/plans/ -name "*.md" -type f | head -5
```

### Step 2: Plan Analysis
- Read the relevant plan file completely
- Understand the current status and priority
- Check the checklist for completed/pending items

### Step 3: Task Execution
- Follow the implementation steps in order
- Update progress in real-time
- Document any deviations or new findings

### Step 4: Plan Updates
- Mark completed items as done
- Add progress logs
- Update status if plan phase is completed

## 📚 Documentation Standards

### Code References
When referencing code in plans, use this format:
- **File paths**: Always use absolute paths from project root
- **Line numbers**: Include when referencing specific lines
- **Code blocks**: Use appropriate language syntax highlighting

### Status Indicators
- ✅ Completed
- 🔄 In Progress  
- ❌ Blocked/Failed
- 📝 Notes/Updates
- 🚨 Critical Issue
- 💡 Suggestion/Idea

### Progress Reporting
Always include:
- What was attempted
- What succeeded/failed
- What's needed next
- Any blockers or questions

## 🚀 Quick Start for Agents

1. **Check for plan files**: `ls docs/plans/`
2. **Read relevant plan**: Focus on "Agent 실행 지침" section
3. **Check current status**: Look at checklist completion
4. **Start with highest priority**: Critical → High → Medium → Low
5. **Update progress**: Add logs as you work
6. **Report completion**: Update final status and success criteria

## 🔍 Troubleshooting

### Common Issues
- **Plan file not found**: Check if task requires creating new plan
- **Outdated plan**: Update timestamps and status before proceeding
- **Conflicting priorities**: Ask user for clarification
- **Incomplete information**: Document what's missing and ask for details

### When to Create New Plans
- Complex multi-step tasks (> 5 steps)
- Architecture changes
- New feature implementations
- Performance optimizations
- Security updates

---

**🤖 This file should be updated by agents as they learn project patterns**  
**Co-Authored-By: Claude <noreply@anthropic.com>**