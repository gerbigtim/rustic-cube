# Project Guidance

## Learning Goal

This project is a Rubik's Cube solver written in Rust, using Bevy to render the cube for debugging and visualization.

The primary goal is to learn Rust well, not to get code written as quickly as possible.

## Default Assistant Behavior

By default, explain rather than implement.

When discussing features, bugs, designs, or implementation ideas, focus on:

- how to model the problem in Rust
- ownership, borrowing, lifetimes, and data layout
- enums, structs, traits, modules, and error handling
- how Bevy concepts map to Rust concepts
- tradeoffs between simple learning-oriented code and more abstract designs
- debugging and testing strategies

Do not modify files or provide full implementations unless explicitly asked.

## Code Snippets

Only provide code snippets when asked.

When providing snippets:

- keep them small
- explain what each Rust concept is doing
- prefer clarity over cleverness
- avoid jumping directly to a complete solution
- show the smallest useful piece of code for the concept being discussed

## Implementation Recommendations

Only give a concrete implementation recommendation when specifically asked.

When recommending an implementation, explain:

- what design choices were made
- why those choices fit Rust
- what alternatives were considered
- what tradeoffs the recommendation has
- how the design could evolve later

## Preferred Workflow

For non-trivial work:

1. Explain the relevant Rust concepts first.
2. Discuss possible approaches.
3. Point out likely mistakes or edge cases.
4. Suggest a small next step.
5. Wait for explicit permission before writing or changing code.

## Project Priorities

Prefer:

- understandable Rust over highly optimized Rust
- simple data structures before advanced abstractions
- explicit state over hidden behavior
- small modules with clear responsibilities
- tests for cube moves and solver logic
- Bevy rendering as a debugging aid, not the core source of truth

Avoid:

- implementing large chunks of the solver unprompted
- hiding important Rust concepts behind abstractions too early
- over-engineering the cube representation before the basics are clear
- treating Bevy ECS as the model for cube logic unless there is a clear reason
