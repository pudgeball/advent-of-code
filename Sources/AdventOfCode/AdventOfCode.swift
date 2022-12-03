protocol Day {
    associatedtype Value
    
    func runSample() throws -> Value

    func runPartOne() throws -> Value
    func runPartTwo() throws -> Value
}

extension Day {
    var enhanced: (any DayEnhanced)? {
        self as? any DayEnhanced
    }
}

protocol DayEnhanced: Day {
    func runSampleTwo() throws -> Value
}

@main
public struct AdventOfCode {
    public static func main() {
        let day: any Day = DayThree()
        
        do {
            print("Sample Input: \(try day.runSample())")
            print("Part One: \(try day.runPartOne())")
            if  let sampleTwo = try? day.enhanced?.runSampleTwo() {
                print("Sample Input: \(sampleTwo)")
            }
            print("Part Two: \(try day.runPartTwo())")
        } catch {
            print("Unable to run puzzle. \(error)")
        }
    }
}
