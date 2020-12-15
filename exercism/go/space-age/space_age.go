package space

type Planet string

func Age(sec float64, planet Planet) float64 {
    var age float64 = sec / 31557600
    switch planet {
    case "Mercury":
        return age / 0.2408467
    case "Venus":
        return age / 0.61519726
    case "Earth":
        return age / 1
    case "Mars":
        return age / 1.8808158
    case "Jupiter":
        return age / 11.862615
    case "Saturn":
        return age / 29.447498
    case "Uranus":
        return age / 84.016846
    case "Neptune":
        return age / 164.79132
    }
    return 0.0
}
