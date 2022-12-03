//
//  Day.swift
//  
//
//  Created by Nick McGuire on 2022-12-03.
//

protocol Day {
    associatedtype Value

    func partOne(_ input: PuzzleInput) throws -> Value
    func partTwo(_ input: PuzzleInput) throws -> Value
    
    var puzzleInput: PuzzleInput { get }
}

extension Day {
    func partOne() throws -> Value {
        try partOne(self.puzzleInput)
    }
    
    func partTwo() throws -> Value {
        try partOne(self.puzzleInput)
    }
}
