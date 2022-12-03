//
//  File.swift
//  
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

public final class DayOne: Day {
    private enum DayOneError: Error {
        case noElfFound
        case noInputPuzzle
    }
    
    private struct Elf {
        fileprivate let calories: Int
        
        public init(calorieInput: [String]) {
            self.calories = calorieInput.compactMap(Int.init).reduce(0, +)
        }
    }
    
    public func runSample() throws -> Int {
        try getCaloricElf("""
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
        """).calories
    }
    
    public func runInput() throws -> Int {
        
        let inputPath = URL(fileURLWithPath: #file).deletingLastPathComponent().appending(path: "Input.txt")
        guard let inputFile = String(data: try Data(contentsOf: inputPath), encoding: .utf8) else {
            throw DayOneError.noInputPuzzle
        }
        return try getCaloricElf(inputFile).calories
    }
    
    private func getCaloricElf(_ input: String) throws -> Elf {
        let lines = input.split(separator: "\n\n")
        let elves = lines.map() { Elf(calorieInput: $0.split(separator: "\n").map(String.init)) }
        guard let elf = elves.sorted(by: { $0.calories > $1.calories }).first else {
            throw DayOneError.noElfFound
        }
        return elf
    }
}
