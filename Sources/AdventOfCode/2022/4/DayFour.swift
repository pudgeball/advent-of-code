//
//  DayFour.swift
//  
//
//  Created by Nick McGuire on 2022-12-04.
//

import Foundation

public final class DayFour: Day {
    // MARK: - Structures
    
    private struct SectionAssignment {
        private let firstElf: ClosedRange<Int>
        private let secondElf: ClosedRange<Int>
        
        init?(_ base: ArraySlice<Character>) {
            let parts = base.split(separator: ",")
            let firstPart = String(parts[0])
            let secondPart = String(parts[1])
            
            let createRange = { (b: String) -> ClosedRange<Int>? in
                let parts = b.split(separator: "-")
                guard let n1 = Int(parts[0]), let n2 = Int(parts[1]) else {
                    return nil
                }
                return ClosedRange.init(uncheckedBounds: (n1, n2))
            }
            
            guard let firstElf = createRange(firstPart), let secondElf = createRange(secondPart) else {
                return nil
            }
            self.firstElf = firstElf
            self.secondElf = secondElf
        }
        
        var workOverlaps: Bool {
            firstElf.overlaps(secondElf) || secondElf.overlaps(firstElf)
        }
        
        var workIsDuplicated: Bool {
            firstElf ~= secondElf || secondElf ~= firstElf
        }
    }
    
    // MARK: - Day
    
    var puzzleInput: PuzzleInput = .init()
    
    public func partOne(_ puzzleInput: PuzzleInput) throws -> Int {
        try puzzleInput.value
            .split(separator: "\n")
            .compactMap(SectionAssignment.init)
            .filter(\.workIsDuplicated)
            .count
    }
    
    public func partTwo(_ puzzleInput: PuzzleInput) throws -> Int {
        try puzzleInput.value
            .split(separator: "\n")
            .compactMap(SectionAssignment.init)
            .filter(\.workOverlaps)
            .count
    }
}

extension ClosedRange {
    fileprivate static func ~=(lhs: Self, rhs: Self) -> Bool {
        rhs.clamped(to: lhs) == rhs
    }
}
