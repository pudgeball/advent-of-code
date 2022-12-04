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
        let sampleInput = """
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
        """.puzzleInput
        XCTAssertEqual(try DayOne().partOne(sampleInput), 24_000)
    }
    
    func testDayTwo() throws {
        let sampleInput = """
        A Y
        B X
        C Z
        """.puzzleInput
        
        let day = DayTwo()
        XCTAssertEqual(try day.partOne(sampleInput), 15)
        XCTAssertEqual(try day.partTwo(sampleInput), 12)
    }
    
    func testDayThree() throws {
        let day = DayThree()
        let sampleInput = """
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        """.puzzleInput
        
        XCTAssertEqual(try day.partOne(sampleInput), 157)
        XCTAssertEqual(try day.partTwo(sampleInput), 70)
    }
    
    func testDayFour() throws {
        let day = DayFour()
        let sampleInput = """
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        """.puzzleInput
        
        XCTAssertEqual(try day.partOne(sampleInput), 2)
        XCTAssertEqual(try day.partTwo(sampleInput), 4)
    }
}
