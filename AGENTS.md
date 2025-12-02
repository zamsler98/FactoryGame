# AGENTS.md

## Build, Lint, and Test Commands
- Build all projects: `dotnet build`
- Run desktop app: `dotnet run --project FactoryGame.DesktopGL/FactoryGame.DesktopGL.csproj`
- Run all tests: `dotnet test FactoryGame.Tests/FactoryGame.Tests.csproj`
- Run a single test: `dotnet test FactoryGame.Tests/FactoryGame.Tests.csproj --filter FullyQualifiedName~TestName`

## Code Style Guidelines (C#)
- **PascalCase** for classes, methods, properties, constants
- **camelCase** for local variables and parameters
- **4 spaces** for indentation (no tabs)
- **Allman style** braces (new line)
- Use `var` for locals when type is obvious; explicit types for public APIs
- XML documentation for public classes/methods
- Use **readonly** for immutable fields
- Descriptive names, avoid abbreviations
- Use `this.` only for ambiguity
- Alphabetize and remove unused `using` statements
- Prefer properties over public fields
- Exception handling only for exceptional cases
- Short, focused methods (single responsibility)
- Use **file-scoped namespaces**

## Project Structure
- Core logic: `FactoryGame.Core`
- Tests: `FactoryGame.Tests` (NUnit, modern assertions)
- Platform code: `FactoryGame.DesktopGL`, `FactoryGame.Android`, `FactoryGame.iOS`
- Follow MonoGame framework conventions

Reference: [Microsoft C# Coding Conventions](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/coding-style/coding-conventions)
