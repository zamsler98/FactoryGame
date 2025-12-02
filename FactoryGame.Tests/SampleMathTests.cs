using NUnit.Framework;

namespace FactoryGame.Tests;

public class SampleMathTests
{
    [Test]
    public void Addition_TwoPlusTwo_EqualsFour()
    {
        // Arrange
        var left = 2;
        var right = 2;

        // Act
        var result = left + right;

        // Assert
        Assert.That(result, Is.EqualTo(4));
    }
}
