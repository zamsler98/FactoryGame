using NUnit.Framework;
using FactoryGame.Core;

namespace FactoryGame.Tests;

/// <summary>
/// Unit tests for the BuildingManager placement logic.
/// </summary>
[TestFixture]
public class BuildingManagerTests
{
    [Test]
    public void TryPlaceBuilding_Succeeds_WhenCellIsEmpty()
    {
        var grid = new Grid();
        var manager = new BuildingManager(grid);
        var result = manager.TryPlaceBuilding(BuildingType.Conveyor, 1, 1);
        Assert.That(result, Is.True);
        Assert.That(grid.IsCellOccupied(1, 1), Is.True);
        Assert.That(grid.GetBuilding(1, 1)?.Type, Is.EqualTo(BuildingType.Conveyor));
    }

    [Test]
    public void TryPlaceBuilding_Fails_WhenCellIsOccupied()
    {
        var grid = new Grid();
        var manager = new BuildingManager(grid);
        manager.TryPlaceBuilding(BuildingType.Factory, 2, 2);
        var result = manager.TryPlaceBuilding(BuildingType.Conveyor, 2, 2);
        Assert.That(result, Is.False);
        Assert.That(grid.GetBuilding(2, 2)?.Type, Is.EqualTo(BuildingType.Factory));
    }

    [Test]
    public void TryPlaceBuilding_Fails_WhenOutOfBounds()
    {
        var grid = new Grid();
        var manager = new BuildingManager(grid);
        var result = manager.TryPlaceBuilding(BuildingType.Factory, 100, 100);
        Assert.That(result, Is.False);
    }
}
