//
//  2022.swift
//  
//
//  Created by Nick McGuire on 2022-12-03.
//

import XCTest
@testable import AdventOfCode

final class AoC2022Tests: XCTestCase {
    func testDayOne() throws {
        let day = AoC2022.DayOne(.init(source: """
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        """))
        XCTAssertEqual(try day.partOne(), 24_000)
    }
    
    func testDayTwo() throws {
        let day = AoC2022.DayTwo(.init(source: """
        A Y
        B X
        C Z
        """))
        XCTAssertEqual(try day.partOne(), 15)
        XCTAssertEqual(try day.partTwo(), 12)
    }
    
    func testDayThree() throws {
        let day = AoC2022.DayThree(.init(source: """
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        """))
        XCTAssertEqual(try day.partOne(), 157)
        XCTAssertEqual(try day.partTwo(), 70)
    }
    
    func testDayFour() throws {
        let day = AoC2022.DayFour(.init(source: """
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        """))
        XCTAssertEqual(try day.partOne(), 2)
        XCTAssertEqual(try day.partTwo(), 4)
    }
    
    func testDayFive() throws {
        let day = AoC2022.DayFive(.init(source: """
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
        """))
        XCTAssertEqual(try day.partOne(), "CMZ")
        XCTAssertEqual(try day.partTwo(), "MCD")
    }
    
    func testDaySix() throws {
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "mjqjpqmgbljsphdztnvjfqwrcgsmlb")).partOne(), 7)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "bvwbjplbgvbhsrlpgdmjqwftvncz")).partOne(), 5)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "nppdvjthqldpwncqszvftbrmjlhg")).partOne(), 6)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).partOne(), 10)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).partOne(), 11)
        
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "mjqjpqmgbljsphdztnvjfqwrcgsmlb")).partTwo(), 19)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "bvwbjplbgvbhsrlpgdmjqwftvncz")).partTwo(), 23)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "nppdvjthqldpwncqszvftbrmjlhg")).partTwo(), 23)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).partTwo(), 29)
        XCTAssertEqual(try AoC2022.DaySix(.init(source: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).partTwo(), 26)
    }
    
    func testDaySeven() throws {
        let day = AoC2022.DaySeven(.init(source: """
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        """))
        
        XCTAssertEqual(try day.partOne(), 95437)
        XCTAssertEqual(try day.partTwo(), 24933642)
    }
}
