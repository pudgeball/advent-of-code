//
//  DayOne.swift
//  
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

extension AoC2022 {
    public final class DayOne: Day {
        // MARK: - Structures
        
        private struct Elf {
            fileprivate let calories: Int
            
            public init(calorieInput: [Substring.SubSequence]) {
                self.calories = calorieInput.compactMap({ Int($0) }).reduce(0, +)
            }
        }
        
        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> Int {
            try getElvesByCalories(puzzleInput).first?.calories ?? 0
        }
        
        public func partTwo() throws -> Int {
            try getElvesByCalories(puzzleInput).prefix(3).map(\.calories).reduce(0, +)
        }
        
        private func getElvesByCalories(_ input: PuzzleInput) throws -> [Elf] {
            try input.value
                .split(separator: "\n\n")
                .map() { Elf(calorieInput: $0.split(separator: "\n")) }
                .sorted(by: { $0.calories > $1.calories })
        }
    }
}
