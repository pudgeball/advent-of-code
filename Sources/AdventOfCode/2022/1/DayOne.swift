//
//  DayOne.swift
//  
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

public final class DayOne: Day {
    private enum DayOneError: Error {
        case noElfFound
    }
    
    private struct Elf {
        fileprivate let calories: Int
        
        public init(calorieInput: [String]) {
            self.calories = calorieInput.compactMap(Int.init).reduce(0, +)
        }
    }
    
    public func runSample() throws -> Int {
        try getElvesByCalories("""
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
        """).first?.calories ?? 0
    }
    
    public func runPartOne() throws -> Int {
        try getElvesByCalories(Input.get("./Input.txt")).first?.calories ?? 0
    }
    
    public func runPartTwo() throws -> Int {
        try getElvesByCalories(Input.get("./Input.txt")).prefix(3).map(\.calories).reduce(0, +)
    }

    private func getElvesByCalories(_ input: String) throws -> [Elf] {
        input.split(separator: "\n\n")
            .map() { Elf(calorieInput: $0.split(separator: "\n").map(String.init)) }
            .sorted(by: { $0.calories > $1.calories })
    }
}
