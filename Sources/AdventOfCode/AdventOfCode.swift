import Foundation

@main
public struct AdventOfCode {
    public static func main() {
//        let savedValue = UserDefaults.standard.string(forKey: "last-run-day")
//        print(savedValue)
        
//        print("Year", terminator: ": ")
//        guard let input = readLine(), let yearNum = Int(input), let year = getYear(yearNum) else {
//            print("Not a valid year")
//            exit(-1)
//        }
//
//        print("Day", terminator: ": ")
//        guard let input = readLine(), let dayNum = Int(input), let day = year.day(dayNum) else {
//            print("Not a valid day")
//            exit(-1)
//        }
        
        let day: any Day = AoC2022.DaySeven()
//        (.init(source:
//        """
//            [D]
//        [N] [C]
//        [Z] [M] [P]
//         1   2   3
//
//        move 1 from 2 to 1
//        move 3 from 1 to 3
//        move 2 from 2 to 1
//        move 1 from 1 to 2
//        """))
        
        do {
            print("Part One: \(try day.partOne())")
            print("Part Two: \(try day.partTwo())")
            
//            UserDefaults.standard.set("\(yearNum)-\(dayNum)", forKey: "last-run-day")
        } catch {
            print("Unable to run puzzle. \(error)")
        }
    }
}

func getYear(_ year: Int) -> (any Year.Type)? {
    switch year {
    case 2022: return AoC2022.self
    default: return nil
    }
}
