##
 
Follow the official Microsoft C# Coding Conventions:
 
- Use **PascalCase** for class, method, property, and constant names.
- Use **camelCase** for local variables and method parameters.
- Use **4 spaces** for indentation; do not use tabs.
- Place **braces on a new line** (Allman style).
- Use `var` for local variables when the type is obvious.
- Prefer explicit types for public APIs.
- Use **XML documentation comments** for public classes and methods.
- Use **readonly** for fields that should not change after initialization.
- Avoid abbreviations; use descriptive names.
- Use `this.` only when necessary to resolve ambiguity.
- Organize `using` statements alphabetically and remove unused usings.
- Prefer properties over public fields.
- Use exception handling only for exceptional cases.
- Keep methods short and focused; prefer single responsibility.
- Use **file-scoped namespaces** (e.g., `namespace FactoryGame;`) instead of block-scoped namespaces.
 
## Project Structure
 
- All core game components, logic, and main code reside in the `FactoryGame.Core` project/namespace.
- All unit tests and test-related code are placed in the `FactoryGame.Tests` project/namespace.
- All unit tests use the **NUnit** framework. Use the modern NUnit assertion style: `Assert.That(actual, Is.EqualTo(expected));`.
- Keep a clear separation between core game code and test code; do not mix them in the same project.
- Organize files and folders within each project to reflect their responsibilities and domain.
- The main entry point and game initialization should reference `FactoryGame.Core`.
- The desktop application and platform-specific code reside in the `FactoryGame.DesktopGL` project/namespace.
 
## Framework
 
This game is developed using the **MonoGame** framework. All game logic, rendering, and input handling should follow MonoGame conventions and APIs. Refer to the [MonoGame Documentation](https://docs.monogame.net/) for framework-specific guidance.
 
Reference: [Microsoft C# Coding Conventions](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/coding-style/coding-conventions)
 
## Common Commands
 
- Run the desktop application:
  ```
  dotnet run --project FactoryGame.DesktopGL/FactoryGame.DesktopGL.csproj
  ```
- Run the tests:
  ```
  dotnet test FactoryGame.Tests/FactoryGame.Tests.csproj
  ```
 
 
 

