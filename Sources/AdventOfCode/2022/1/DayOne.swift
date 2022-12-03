//
//  DayOne.swift
//  
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

public final class DayOne: Day {
    var puzzleInput: PuzzleInput = .init()
    
    private enum DayOneError: Error {
        case noElfFound
    }
    
    private struct Elf {
        fileprivate let calories: Int
        
        public init(calorieInput: [String]) {
            self.calories = calorieInput.compactMap(Int.init).reduce(0, +)
        }
    }
    
    public func partOne(_ puzzleInput: PuzzleInput) throws -> Int {
        try getElvesByCalories(puzzleInput).first?.calories ?? 0
    }
    
    public func partTwo(_ puzzleInput: PuzzleInput) throws -> Int {
        try getElvesByCalories(puzzleInput).prefix(3).map(\.calories).reduce(0, +)
    }

    private func getElvesByCalories(_ input: PuzzleInput) throws -> [Elf] {
        try input.value
            .split(separator: "\n\n")
            .map() { Elf(calorieInput: $0.split(separator: "\n").map(String.init)) }
            .sorted(by: { $0.calories > $1.calories })
    }
}
