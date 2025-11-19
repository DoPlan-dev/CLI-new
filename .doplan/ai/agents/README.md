# DoPlan AI Agents

This directory contains AI agent definitions for use with your IDE's AI assistant (Cursor, VS Code Copilot, Gemini CLI, etc.).

## Available Agents

- **@planner** - Senior project planner. Handles idea discussion, refinement, PRD generation, and project planning. **FIRST step** in the workflow.
- **@designer** - UI/UX specialist. Creates design specifications and follows the design system from DPR.
- **@coder** - Implementation specialist. Implements features based on plans and designs.
- **@tester** - QA & Test Automation Specialist. Creates and runs tests using Playwright (MCP), captures screenshots, performs visual regression checks.
- **@reviewer** - Quality assurance. Reviews code and provides feedback. **ONLY reviews AFTER @tester has successfully run all tests.**
- **@devops** - Deployment and infrastructure specialist. Handles deployment **ONLY AFTER @reviewer has approved.**

## How to Activate Agents

### In Cursor
1. Type **@** in the chat to see available agents
2. Select an agent (e.g., **@planner**)
3. Ask your question or request
4. Agents will automatically follow their defined workflows

### In VS Code with Copilot
1. Reference agents in your prompts: "Use **@planner** to help plan this feature"
2. Agents will follow their defined workflows and rules

### In Other IDEs
- Reference agents by name: "@planner", "@coder", etc.
- Agents will follow the workflow rules defined in `.doplan/ai/rules/`

## ⚠️ CRITICAL: Workflow & Rules

**ALL agents MUST follow these rules:**

1. **Workflow Rules:** Read and obey `.doplan/ai/rules/workflow.mdc`
   - This defines the perfect workflow sequence: Plan → Design → Code → Test → Review → Deploy
   - Each agent has a specific position in this workflow
   - **DO NOT skip steps or work out of order**

2. **Communication Rules:** Read and obey `.doplan/ai/rules/communication.mdc`
   - This defines how agents must interact and hand off tasks
   - **ALWAYS tag the next agent when your work is complete**
   - **ALWAYS wait for the previous agent to finish before starting**

3. **Design Rules:** Designers and Coders MUST follow `.doplan/ai/rules/design_rules.mdc`
   - This contains the design system from DPR (Design Preferences & Requirements)
   - Use design tokens for all styling
   - Follow component guidelines

## Perfect Workflow Sequence

The DoPlan workflow follows this exact sequence:

1. **@planner** → Discuss idea, refine, generate PRD, create plan
   - Creates phase and feature folders: `doplan/01-user-authentication/01-login-with-email/`
   - Uses numbered and slugified names for human readability and clear ordering

2. **@designer** → Create design specifications
   - **MUST wait for @planner to finish**
   - Follows design_rules.mdc from DPR
   - Creates design.md for each feature

3. **@coder** → Implement features
   - **MUST wait for @planner AND @designer to finish**
   - Follows plan.md, design.md, and tasks.md
   - **MUST tag @tester when work is ready for testing**

4. **@tester** → Create and run tests
   - **ONLY begins when tagged by @coder**
   - Uses Playwright (MCP) for end-to-end tests
   - **Captures screenshots** to `.doplan/artifacts/screenshots/{phase-name}/{feature-name}.png`
   - Performs visual regression checks
   - **MUST tag @reviewer with test report (pass or fail)**

5. **@reviewer** → Review code quality
   - **ONLY reviews AFTER @tester has successfully run all tests**
   - Reviews against plan.md and design.md
   - **MUST tag @devops when approved, or request changes from @coder**

6. **@devops** → Handle deployment
   - **ONLY begins AFTER @reviewer has approved**
   - Configures deployment pipelines
   - Reports deployment status back to team

## Multi-Agent Conversations

Agents can work together in conversations. Example:

```
User: @planner Help me plan a user authentication feature

@planner: [Creates plan, generates PRD, creates folder structure]
         ✅ Planning complete. Created doplan/01-user-authentication/01-login-with-email/
         Tagging @designer to create design specifications.

@designer: [Creates design.md following design_rules.mdc]
          ✅ Design complete. Tagging @coder to begin implementation.

@coder: [Implements login functionality]
        ✅ Implementation complete. Tagging @tester to run tests.

@tester: [Runs Playwright tests, captures screenshots]
         ✅ All tests passed. Screenshot saved to .doplan/artifacts/screenshots/01-user-authentication/01-login-with-email.png
         Tagging @reviewer for code review.

@reviewer: [Reviews code against plan and design]
           ✅ Code review passed. Tagging @devops for deployment.

@devops: [Configures deployment]
         ✅ Deployment configured and ready.
```

## Folder Structure Requirements

**CRITICAL:** When creating phase and feature folders, **ALWAYS use numbered and slugified names:**

- ✅ **Correct:** `doplan/01-user-authentication/01-login-with-email/`
- ✅ **Correct:** `doplan/02-dashboard/01-user-profile/`
- ❌ **Wrong:** `doplan/user-authentication/login/` (no numbers)
- ❌ **Wrong:** `doplan/1-auth/1-login/` (not zero-padded)

**Format:** `{##}-{slugified-name}`
- Use two-digit numbers with leading zeros (01, 02, 03...)
- Use kebab-case for names
- This provides both human readability and clear ordering

## File Structure

- `README.md` - This file (usage guide)
- `planner.agent.md` - Planner agent definition
- `coder.agent.md` - Coder agent definition
- `designer.agent.md` - Designer agent definition
- `reviewer.agent.md` - Reviewer agent definition
- `tester.agent.md` - Tester agent definition (with Playwright/MCP)
- `devops.agent.md` - DevOps agent definition

## Integration

These agents are automatically linked to your IDE:
- **Cursor:** `.cursor/agents/` → `.doplan/ai/agents/` (symlinked)
- **VS Code:** Available via Copilot Chat
- **Gemini CLI:** Available via command references
- **Other IDEs:** See `.doplan/guides/` for setup instructions

## Commands & Workflows

Each agent supports specific commands. See individual agent files for details:
- **@planner:** /Plan, /Plan:Phase, /Plan:Reorder, /Plan:Dependencies
- **@designer:** /Design, /Design:Review
- **@coder:** /Implement
- **@tester:** /Test, /Test:Visual
- **@reviewer:** /Review
- **@devops:** /Deploy, /Deploy:Configure

## Best Practices

1. **Always follow the workflow sequence** - Don't skip steps
2. **Tag the next agent** when your work is complete
3. **Wait for previous agents** to finish before starting
4. **Read workflow.mdc and communication.mdc** before starting work
5. **Use numbered folder structure** for phases and features
6. **Follow design_rules.mdc** for all UI/UX work
7. **Capture screenshots** for all completed features
8. **Update progress files** after each action

## Troubleshooting

**Q: Agent not appearing in IDE?**
- Ensure `.doplan/ai/agents/` directory exists
- For Cursor: Check that symlinks are created in `.cursor/agents/`
- Run `doplan install` to regenerate agents

**Q: Agent not following workflow?**
- Ensure `.doplan/ai/rules/workflow.mdc` exists
- Remind the agent to read workflow.mdc
- Check that communication.mdc is present

**Q: Screenshots not being saved?**
- Ensure `.doplan/artifacts/screenshots/` directory exists
- Check that Playwright (MCP) is configured
- Verify @tester agent has screenshot capture instructions
