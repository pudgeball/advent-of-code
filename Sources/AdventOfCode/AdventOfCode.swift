protocol Day {
    associatedtype Value
    
    func runSample() throws -> Value
    func runInput() throws -> Value
}

@main
public struct AdventOfCode {
    public static func main() {
        let day: any Day = DayOne()
        
        do {
            print("Sample Input: \(try day.runSample())")
            print("Puzzle Input: \(try day.runInput())")
        } catch {
            print("Unable to run puzzle. \(error)")
        }
    }
}
