using System;
using FactoryGame.Core;
using Microsoft.Xna.Framework;
using NUnit.Framework;

namespace FactoryGame.Tests;

/// <summary>
/// Unit tests for Player class.
/// </summary>
[TestFixture]
public class PlayerTests
{
    private Player player;
    private Vector2 initialPosition;

    [SetUp]
    public void SetUp()
    {
        initialPosition = new Vector2(100, 100);
        player = new Player(initialPosition);
    }

    [TearDown]
    public void TearDown()
    {
        player?.Dispose();
    }

    [Test]
    public void Constructor_WithValidPosition_SetsPositionCorrectly()
    {
        // Arrange
        var expectedPosition = new Vector2(50, 75);

        // Act
        var testPlayer = new Player(expectedPosition);

        // Assert
        var positionField = testPlayer.GetType()
            .GetField("position", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
        var actualPosition = (Vector2)positionField!.GetValue(testPlayer)!;
        Assert.That(actualPosition, Is.EqualTo(expectedPosition));
    }

    [Test]
    public void Update_WithNoInput_DoesNotChangePosition()
    {
        // Arrange
        var gameTime = new GameTime(TimeSpan.Zero, TimeSpan.FromSeconds(0.016f)); // ~60 FPS
        
        var positionField = player.GetType()
            .GetField("position", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
        var initialPos = (Vector2)positionField!.GetValue(player)!;

        // Act
        var mockInputManager = new InputManager();
        player.Update(gameTime, mockInputManager);

        // Assert
        var finalPos = (Vector2)positionField.GetValue(player)!;
        Assert.That(finalPos.X, Is.EqualTo(initialPos.X));
        Assert.That(finalPos.Y, Is.EqualTo(initialPos.Y));
    }

    [Test]
    public void Update_WithMovement_RespectsUpperBoundaries()
    {
        // Arrange
        var gameTime = new GameTime(TimeSpan.Zero, TimeSpan.FromSeconds(1.0f));
        
        // Set player position at the maximum valid position
        var positionField = player.GetType()
            .GetField("position", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
        positionField!.SetValue(player, new Vector2(
            GameConstants.WindowWidth - GameConstants.PlayerTextureSize, 
            GameConstants.WindowHeight - GameConstants.PlayerTextureSize));

        // Act
        var mockInputManager = new InputManager();
        player.Update(gameTime, mockInputManager);

        // Assert - player should not go beyond boundaries
        var finalPos = (Vector2)positionField.GetValue(player)!;
        Assert.That(finalPos.X, Is.LessThanOrEqualTo(GameConstants.WindowWidth - GameConstants.PlayerTextureSize));
        Assert.That(finalPos.Y, Is.LessThanOrEqualTo(GameConstants.WindowHeight - GameConstants.PlayerTextureSize));
    }

    [Test]
    public void Update_WithMovement_RespectsLowerBoundaries()
    {
        // Arrange
        var gameTime = new GameTime(TimeSpan.Zero, TimeSpan.FromSeconds(1.0f));
        
        // Set player position at origin
        var positionField = player.GetType()
            .GetField("position", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
        positionField!.SetValue(player, Vector2.Zero);

        // Act
        var mockInputManager = new InputManager();
        player.Update(gameTime, mockInputManager);

        // Assert - player should not go below 0
        var finalPos = (Vector2)positionField.GetValue(player)!;
        Assert.That(finalPos.X, Is.GreaterThanOrEqualTo(0));
        Assert.That(finalPos.Y, Is.GreaterThanOrEqualTo(0));
    }

    [Test]
    public void PlayerSpeed_IsCorrectValue()
    {
        // Arrange & Act
        var speedField = typeof(Player)
            .GetField("speed", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
        var testPlayer = new Player(Vector2.Zero);
        var speed = speedField!.GetValue(testPlayer);

        // Assert
        Assert.That(speed, Is.EqualTo(GameConstants.PlayerSpeed));
    }

    
}