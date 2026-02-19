# Data Flow Architecture

## Learning Cycle Flow

```
┌─────────────────┐
│  Start System   │
└────────┬────────┘
         │
         v
┌─────────────────────────┐
│   Question Agent        │
│   - Generate question   │
│   - Write to file       │
└────────┬────────────────┘
         │
         v
    questions.txt
         │
         v
┌─────────────────────────┐
│   Answer Agent          │
│   - Read question       │
│   - Generate answer     │
│   - Write to file       │
└────────┬────────────────┘
         │
         v
    answers.txt
         │
         v
┌─────────────────────────┐
│   Learning Agent        │
│   - Read Q&A pair       │
│   - Process knowledge   │
│   - Update KB           │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│   Continue or Stop?     │
│   (User control)        │
└────────┬────────────────┘
         │
    Loop back or Exit
```

## File Data Flow

### questions.txt Format
```
[TIMESTAMP] Q1: Question text here
[TIMESTAMP] Q2: Next question text
```

### answers.txt Format
```
[TIMESTAMP] A1: Answer corresponding to Q1
[CODE_BLOCK]
impl MyStruct {
    fn example() { }
}
[/CODE_BLOCK]

[TIMESTAMP] A2: Answer corresponding to Q2
```

### knowledge_base.json Structure
```json
{
  "version": "1.0",
  "last_updated": "timestamp",
  "qa_pairs": [
    {
      "question": "...",
      "answer": "...",
      "code_examples": ["..."],
      "tags": ["module", "structure"],
      "learned_at": "timestamp"
    }
  ],
  "patterns": [
    {
      "name": "error_handling",
      "description": "...",
      "code_template": "..."
    }
  ],
  "project_templates": [
    {
      "type": "cli_app",
      "structure": {...},
      "dependencies": [...]
    }
  ]
}
```

## Knowledge Processing Pipeline

```
Q&A Pair
    │
    v
┌─────────────────┐
│   Parser        │ - Extract structured data
└────────┬────────┘
         │
         v
┌─────────────────┐
│   Analyzer      │ - Identify patterns
└────────┬────────┘
         │
         v
┌─────────────────┐
│   Integrator    │ - Merge with existing knowledge
└────────┬────────┘
         │
         v
┌─────────────────┐
│   Storage       │ - Persist to knowledge_base.json
└─────────────────┘
```

## Project Generation Flow

```
User Request: "Create a CLI tool"
    │
    v
┌──────────────────────────┐
│  Query Knowledge Base    │
└────────┬─────────────────┘
         │
         v
┌──────────────────────────┐
│  Select Relevant         │
│  Patterns & Templates    │
└────────┬─────────────────┘
         │
         v
┌──────────────────────────┐
│  Generate Project Files  │
│  - Cargo.toml            │
│  - src/main.rs           │
│  - src/modules/*.rs      │
└────────┬─────────────────┘
         │
         v
┌──────────────────────────┐
│  Write to Disk           │
└──────────────────────────┘
```
