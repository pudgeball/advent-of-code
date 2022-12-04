//
//  Day.swift
//  
//
//  Created by Nick McGuire on 2022-12-03.
//

protocol Day {
    associatedtype Value
    
    init(_ puzzleInput: PuzzleInput)

    func partOne() throws -> Value
    func partTwo() throws -> Value
    
    var puzzleInput: PuzzleInput { get }
}
