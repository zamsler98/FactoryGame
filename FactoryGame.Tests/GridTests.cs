using NUnit.Framework;
using FactoryGame.Core;

namespace FactoryGame.Tests;

/// <summary>
/// Unit tests for the Grid and building placement logic.
/// </summary>
[TestFixture]
public class GridTests
{
    [Test]
    public void PlaceBuilding_Succeeds_WhenCellIsEmpty()
    {
        var grid = new Grid();
        var building = new Building(BuildingType.Factory, 0, 0);
        var result = grid.PlaceBuilding(building, 0, 0);
        Assert.That(result, Is.True);
        Assert.That(grid.IsCellOccupied(0, 0), Is.True);
        Assert.That(grid.GetBuilding(0, 0), Is.EqualTo(building));
    }

    [Test]
    public void PlaceBuilding_Fails_WhenCellIsOccupied()
    {
        var grid = new Grid();
        var building1 = new Building(BuildingType.Factory, 0, 0);
        var building2 = new Building(BuildingType.Conveyor, 0, 0);
        grid.PlaceBuilding(building1, 0, 0);
        var result = grid.PlaceBuilding(building2, 0, 0);
        Assert.That(result, Is.False);
        Assert.That(grid.GetBuilding(0, 0), Is.EqualTo(building1));
    }

    [Test]
    public void PlaceBuilding_Fails_WhenOutOfBounds()
    {
        var grid = new Grid();
        var building = new Building(BuildingType.Factory, -1, 0);
        var result = grid.PlaceBuilding(building, -1, 0);
        Assert.That(result, Is.False);
        Assert.That(grid.IsCellOccupied(-1, 0), Is.False);
    }
}
