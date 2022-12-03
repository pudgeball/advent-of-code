//
//  DayTwo.swift
//
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

public final class DayTwo: Day {
    var puzzleInput: PuzzleInput = .init()
    
    enum DayTwoError: Error {
        case invalidCharacter
    }
    
    enum GameState {
        case win
        case tie
        case loss
    }
    
    enum Shape: Int {
        case rock = 1
        case paper = 2
        case scissors = 3
        
        func compare(_ rhs: Shape) -> GameState {
            if self == rhs {
                return .tie
            }
            if self == .rock && rhs == .scissors
                || self == .paper && rhs == .rock
                || self == .scissors && rhs == .paper {
                return .win
            }
            return .loss
        }
        
        var desiredOutcome: GameState {
            switch self {
            case .rock:
                return .loss
            case .paper:
                return .tie
            case .scissors:
                return .win
            }
        }
    }
    
    public func partOne(_ puzzleInput: PuzzleInput) throws -> Int {
        try getTotalScore(puzzleInput)
    }
    
    public func partTwo(_ puzzleInput: PuzzleInput) throws -> Int {
        try getPartTwoScore(puzzleInput)
    }

    private func getTotalScore(_ input: PuzzleInput) throws -> Int {
        try input.value
            .split(separator: "\n")
            .map() { line in
                let theirs = try! line.first!.choiceValue
                let yours = try! line.last!.choiceValue
                
                switch yours.compare(theirs) {
                case .win:
                    return yours.rawValue + 6
                case .tie:
                    return yours.rawValue + 3
                case .loss:
                    return yours.rawValue
                }
            }
            .reduce(0, +)
    }
    
    private func getPartTwoScore(_ input: PuzzleInput) throws -> Int {
        try input.value
            .split(separator: "\n")
            .map() { line in
                let theirs = try! line.first!.choiceValue
                let yours = try! line.last!.choiceValue
                
                switch yours {
                case .rock: // lose
                    if theirs == .scissors {
                        return Shape.paper.rawValue
                    } else if theirs == .paper {
                        return Shape.rock.rawValue
                    } else {
                        return Shape.scissors.rawValue
                    }
                case .paper: // draw
                    return 3 + theirs.rawValue
                case .scissors: // win
                    if theirs == .rock {
                        return Shape.paper.rawValue + 6
                    } else if theirs == .paper {
                        return Shape.scissors.rawValue + 6
                    } else {
                        return Shape.rock.rawValue + 6
                    }
                }
            }
            .reduce(0, +)
    }
}

extension Character {
    fileprivate var choiceValue: DayTwo.Shape {
        get throws {
            switch self {
            case "A", "X":
                return .rock
            case "B", "Y":
                return .paper
            case "C", "Z":
                return .scissors
            default:
                throw DayTwo.DayTwoError.invalidCharacter
            }
        }
    }
}
