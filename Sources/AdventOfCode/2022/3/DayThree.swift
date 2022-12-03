//
//  DayThree.swift
//
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

public final class DayThree: DayEnhanced {
    public func runSample() throws -> Int {
        try getPriorities("""
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        """)
    }
    
    public func runSampleTwo() throws -> Int {
        try getMorePriorities("""
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        """)
    }

    public func runPartOne() throws -> Int {
        try getPriorities(Input.get("./Input.txt"))
    }
    
    public func runPartTwo() throws -> Int {
        try getMorePriorities(Input.get("./Input.txt"))
    }

    private func getPriorities(_ input: String) throws -> Int {
        input.split(separator: "\n")
            .map() { line in
                let halfCount = line.count / 2
                let firstHalf = Set(line.dropLast(halfCount))
                let secondHalf = Set(line.dropFirst(halfCount))

                let char = firstHalf.intersection(secondHalf).first!
                return char.priority
            }
            .reduce(0 , +)
    }
    
    private func getMorePriorities(_ input: String) throws -> Int {
        input.split(separator: "\n")
            .chunked(into: 3)
            .compactMap { group in
                group
                    .map(Set.init)
                    .reduce(Set<Character>()) { $0.count == 0 ? $1 : $0.intersection($1) }
            }
            .map { ac in
                ac.first!.priority
            }
            .reduce(0, +)
    }
}

extension Array {
    fileprivate func chunked(into size: Int) -> [[Element]] {
        return stride(from: 0, to: count, by: size).map {
            Array(self[$0 ..< Swift.min($0 + size, count)])
        }
    }
}

extension Character {
    fileprivate var priority: Int {
        if self >= "a" && self <= "z" { // 1-26
            return (Int(self.asciiValue!) - 97) + 1
        } else if self >= "A" && self <= "Z" { // 27-52
            return (Int(self.asciiValue!) - 65) + 27
        }
        return 0
    }
}
