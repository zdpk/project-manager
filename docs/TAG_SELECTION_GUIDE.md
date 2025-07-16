# Tag Selection Interface Guide

This guide covers PM's new two-step tag selection interface, designed to provide a cleaner and more intuitive experience when adding projects.

## Overview

The tag selection interface has been redesigned to eliminate confusion and provide a more intuitive workflow. Instead of a single complex interface, the process is now split into two clear steps:

1. **Action Selection**: Choose what you want to do
2. **Tag Selection**: Select or create tags based on your choice

## Interface Flow

### Step 1: Action Selection

When you add a project with `pm add`, you'll first see:

```
? What would you like to do?
  > Create Project [project-name] (without tags)
    Add tags to this project
    Create new tag and add to project
```

This step uses a simple select interface (no checkboxes) to clearly separate different actions.

### Step 2: Conditional Tag Selection

Based on your choice in Step 1, you'll see different interfaces:

## Case Examples

### Case 1: Create Project Without Tags

**When to use**: You want to quickly add a project without any tags.

```bash
$ pm add ./my-simple-script

? What would you like to do?
  > Create Project [my-simple-script] (without tags)
    Add tags to this project
    Create new tag and add to project

✅ Successfully added project 'my-simple-script'
   Path: /Users/you/projects/my-simple-script
```

**Result**: Project is created immediately with no tags.

---

### Case 2: Add Existing Tags

**When to use**: You want to categorize your project using tags that already exist.

```bash
$ pm add ./react-dashboard

? What would you like to do?
    Create Project [react-dashboard] (without tags)
  > Add tags to this project
    Create new tag and add to project

🏷️ Select tags for this project (type to filter):
  [ ] frontend (3 projects)
  [ ] react (2 projects)
  [ ] typescript (5 projects)
  [ ] dashboard (1 projects)
  [ ] web (4 projects)
```

**Interactive features:**
- **Arrow keys** to navigate up/down
- **Space** to select/deselect tags
- **Type** to filter tags (e.g., type "react" to show only tags containing "react")
- **Enter** to confirm selection

```bash
# After selecting frontend and react:
🏷️ Select tags for this project (type to filter): frontend, react
  [x] frontend (3 projects)
  [x] react (2 projects)
  [ ] typescript (5 projects)
  [ ] dashboard (1 projects)
  [ ] web (4 projects)

✅ Successfully added project 'react-dashboard' with tags: frontend, react
   Path: /Users/you/projects/react-dashboard
```

**Filtering example:**
```bash
# Type "type" to filter:
🏷️ Select tags for this project (type to filter): type
  [ ] typescript (5 projects)

# Type "fr" to filter:
🏷️ Select tags for this project (type to filter): fr  
  [ ] frontend (3 projects)
```

---

### Case 3: Create New Tags

**When to use**: You need to create new tags for your project, optionally adding existing ones too.

```bash
$ pm add ./api-microservice

? What would you like to do?
    Create Project [api-microservice] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: backend
? Add another new tag? Yes

✨ Create new tag: microservice  
? Add another new tag? No

? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [ ] node (1 projects)
  [ ] api (2 projects)
  [ ] docker (3 projects)
  > [x] node
    [x] api

✅ Successfully added project 'api-microservice' with tags: backend, microservice, node, api
   Path: /Users/you/projects/api-microservice
```

**Flow breakdown:**
1. Create first new tag: "backend"
2. Option to create another: "microservice" 
3. Option to add existing tags: Select "node" and "api"
4. Final result: 4 tags total (2 new + 2 existing)

---

### Case 4: No Existing Tags Scenario

**When to use**: This is your first project or no tags exist yet.

```bash
$ pm add ./first-project

? What would you like to do?
    Create Project [first-project] (without tags)
  > Add tags to this project
    Create new tag and add to project

# If "Add tags to this project" is selected:
ℹ️  No existing tags found. Creating project without tags.

✅ Successfully added project 'first-project'
   Path: /Users/you/projects/first-project
```

Alternatively, choose "Create new tag and add to project":

```bash
? What would you like to do?
    Create Project [first-project] (without tags)
    Add tags to this project
  > Create new tag and add to project

✨ Create new tag: personal
? Add another new tag? No

? Add existing tags as well? No
# (No existing tags available anyway)

✅ Successfully added project 'first-project' with tags: personal
   Path: /Users/you/projects/first-project
```

## Advanced Features

### Tag Filtering

When selecting from existing tags, you can type to filter the list:

```bash
🏷️ Select tags for this project (type to filter): java
  [ ] java (3 projects)
  [ ] javascript (2 projects)
```

**Filtering is smart:**
- Matches any part of the tag name
- Case insensitive
- Real-time filtering as you type
- Preserves selection state when filtering

### Multiple Tag Creation

You can create multiple new tags in sequence:

```bash
✨ Create new tag: database
? Add another new tag? Yes

✨ Create new tag: postgresql  
? Add another new tag? Yes

✨ Create new tag: migration
? Add another new tag? No
```

### Combining New and Existing Tags

The interface allows flexible combinations:

```bash
# Create new tags first:
✨ Create new tag: machine-learning
✨ Create new tag: pytorch

# Then optionally add existing tags:
? Add existing tags as well? Yes

🏷️ Select tags for this project (type to filter):
  [x] python (5 projects)
  [x] research (2 projects)

# Final result: machine-learning, pytorch, python, research
```

## Tips and Best Practices

### 1. Use Descriptive Actions
- **"Create Project without tags"**: For quick project addition
- **"Add tags to this project"**: When you know tags exist
- **"Create new tag"**: For new categories or specific project types

### 2. Tag Naming Conventions
- Use lowercase with hyphens: `machine-learning`, `web-app`
- Be descriptive but concise: `frontend` not `front-end-development`
- Group related concepts: `react`, `vue`, `angular` for frameworks

### 3. Efficient Filtering
- Type partial words: "back" to find "backend"
- Use unique prefixes: "ml" for "machine-learning"
- Clear and retype if filter becomes too narrow

### 4. Batch Operations
For multiple projects (`pm add *`), the interface is streamlined:
- No interactive tag selection (to avoid repetition)
- Projects are added without tags by default
- Use `pm tag add project-name tag1 tag2` afterward if needed

## Troubleshooting

### Empty Tag Selection
If you see no tags in the selection list:
```bash
ℹ️  No existing tags found. Creating project without tags.
```
This is normal for new PM installations or when no projects have tags yet.

### Filtering Shows No Results
If your filter doesn't match anything:
- Clear the filter and try again
- Check spelling
- Use broader terms

### Accidental Selection
- Use **Space** to toggle selections on/off
- Press **Ctrl+C** to cancel and start over
- Review selections before pressing **Enter**

## Comparison with Previous Interface

### Before (Old Interface)
```bash
🏷️ Tags: (rust, cli, tools)  # Confusing brackets
> Add tags to this project? Yes
? Enter new tag (or press Enter to finish): [Use descriptive tags like 'rust', 'frontend', 'work']
```

### After (New Interface)  
```bash
? What would you like to do?
  > Add tags to this project

🏷️ Select tags for this project (type to filter):
  [x] rust (3 projects)
  [x] cli (2 projects)
```

**Improvements:**
- ✅ Clear separation of actions vs. tag selection
- ✅ Visual feedback with checkboxes and counts
- ✅ Real-time filtering with clear instructions
- ✅ No confusing bracket notation
- ✅ Flexible workflow supporting all use cases

## Related Commands

After creating projects with tags, you can:

```bash
# List projects by tags
pm ls --tags rust,cli

# Add more tags later
pm tag add project-name new-tag

# Remove tags
pm tag remove project-name old-tag

# List all tags
pm tag list
```

For more information, see [COMMANDS.md](COMMANDS.md).