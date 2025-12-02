using NUnit.Framework;
using System;
using FactoryGame.Core;

namespace FactoryGame.Tests;

/// <summary>
/// Unit tests for the GridOverlay class.
/// </summary>
[TestFixture]
public class GridOverlayTests
{
    [Test]
    public void Constructor_WithNullGraphicsDevice_ThrowsArgumentNullException()
    {
        // Act & Assert
        Assert.Throws<ArgumentNullException>(() => new GridOverlay(null));
    }
}