//
//  2022.swift
//  
//
//  Created by Nick McGuire on 2022-12-04.
//

import Foundation

enum AoC2022: Year {
    static func day(_ day: Int) -> (any Day)? {
        switch day {
        case 1: return DayOne()
        case 2: return DayTwo()
        case 3: return DayThree()
        case 4: return DayFour()
        default: return nil
        }
    }
}
