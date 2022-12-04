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
}
