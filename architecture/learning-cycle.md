# Learning Cycle Details

## Teaching Phase

The teaching phase is where the two teaching agents (Question and Answer agents) work together to build knowledge that the Learning Agent will consume.

### Phase 1: Question Generation

**Question Agent Behavior**:
1. Start with fundamental Rust concepts
2. Progress to intermediate topics
3. Cover project-specific patterns
4. Generate questions about:
   - Syntax and language features
   - Project structure and organization
   - Common patterns and idioms
   - Error handling strategies
   - Performance considerations
   - Testing approaches
   - Documentation practices

**Question Categories**:
- **Fundamentals**: ownership, borrowing, lifetimes
- **Structure**: modules, crates, workspaces
- **Patterns**: traits, generics, macros
- **Practical**: CLI args, file I/O, concurrency
- **Ecosystem**: popular crates, tooling

### Phase 2: Answer Generation

**Answer Agent Behavior**:
1. Read the latest questions from `questions.txt`
2. Provide detailed, accurate answers
3. Include practical code examples
4. Explain reasoning and best practices
5. Reference Rust documentation concepts

**Answer Quality Criteria**:
- Accuracy: Information must be correct
- Completeness: Cover the topic thoroughly
- Clarity: Use clear, understandable language
- Examples: Include working code samples
- Context: Explain why, not just how

### Phase 3: Knowledge Absorption

**Learning Agent Behavior**:
1. Monitor both `questions.txt` and `answers.txt`
2. Match questions with their answers
3. Parse and structure the information
4. Extract key concepts and patterns
5. Store in knowledge base
6. Update internal models

**Learning Strategies**:
- **Pattern Recognition**: Identify recurring code patterns
- **Template Extraction**: Pull out reusable code templates
- **Concept Mapping**: Build relationships between concepts
- **Confidence Scoring**: Rate understanding of each topic

## Stopping Criteria

The learning cycle continues until:
1. User explicitly stops the teaching process
2. A predefined number of Q&A pairs is reached
3. Error conditions are encountered

## Learning Metrics

Track progress with:
- Total Q&A pairs processed
- Topics covered
- Patterns identified
- Templates created
- Confidence scores per topic area

## Example Learning Session

```
Iteration 1:
Q: How do you create a new Rust project?
A: Use 'cargo new project_name'...
Learning: Store command pattern, project initialization template

Iteration 2:
Q: How do you add dependencies to a Rust project?
A: Edit Cargo.toml [dependencies] section...
Learning: Update Cargo.toml template, dependency management pattern

Iteration 3:
Q: How do you structure a multi-module Rust project?
A: Use mod declarations and src/ subdirectories...
Learning: Module system pattern, project organization template

... continues until stopped ...

Result: Knowledge base contains:
- Project initialization patterns
- Dependency management
- Module organization
- [many more patterns]
```

## Continuous Improvement

Over time, the Learning Agent:
- Reinforces previously learned concepts
- Identifies conflicts or contradictions
- Updates patterns based on new information
- Improves code generation quality
- Adapts to new Rust features and best practices
