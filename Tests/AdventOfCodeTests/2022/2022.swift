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
        let day = DayOne()
        XCTAssertEqual(try day.runSample(), 24000)
    }
    
    func testDayTwo() throws {
        let day = DayTwo()
        XCTAssertEqual(try day.runSample(), 15)
        XCTAssertEqual(try day.runSampleTwo(), 12)
    }
    
    func testDayThree() throws {
        let day = DayThree()
        XCTAssertEqual(try day.runSample(), 157)
        XCTAssertEqual(try day.runSampleTwo(), 70)
    }
}
