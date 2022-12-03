protocol Day {
    associatedtype Value
    
    func runSample() throws -> Value
    func runPartOne() throws -> Value
    func runPartTwo() throws -> Value
}

@main
public struct AdventOfCode {
    public static func main() {
        let day: any Day = DayOne()
        
        do {
            print("Sample Input: \(try day.runSample())")
            print("Part One: \(try day.runPartOne())")
            print("Part Two: \(try day.runPartTwo())")
        } catch {
            print("Unable to run puzzle. \(error)")
        }
    }
}
