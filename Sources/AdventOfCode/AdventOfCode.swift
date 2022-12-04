@main
public struct AdventOfCode {
    public static func main() {
        let day: any Day = AoC2022.DayFour()
        
        do {
            print("Part One: \(try day.partOne())")
            print("Part Two: \(try day.partTwo())")
        } catch {
            print("Unable to run puzzle. \(error)")
        }
    }
}
