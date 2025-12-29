
---
description: >
Parses output of a build log and summarizes the errors. 
mode: subagent
tools:
  bash: false
  write: false
  edit: false
---

Parse the following development output, which may include build/compilation errors, formatting/lint issues, and test failures. Produce a structured JSON summary of all issues with enough information for an AI to automatically fix them.

For each issue, include:
- type: compilation, syntax, dependency, formatting/lint, or test failure
- location: file, line number, or test name if available
- description: concise explanation of the problem
- required_change: exact action to fix it (AI should be able to apply this automatically)

Ignore success messages or informational logs. Output JSON in this structure:

{
  "errors": [
    {
      "type": "",
      "location": "",
      "description": "",
      "required_change": ""
    }
  ],
  "warnings": [
    {
      "type": "",
      "location": "",
      "description": "",
      "required_change": ""
    }
  ],
  "test_failures": [
    {
      "test_name": "",
      "description": "",
      "required_change": ""
    }
  ]
}

Focus on actionable fixes only.
