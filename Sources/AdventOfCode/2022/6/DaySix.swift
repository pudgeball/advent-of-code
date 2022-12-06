//
//  DaySix.swift
//  
//
//  Created by Nick McGuire on 2022-12-05.
//

import Foundation

extension AoC2022 {
    public final class DaySix: Day {
        // MARK: - Structures
        
        struct Stack<T> where T : Equatable, T : Hashable {
            private var backing: [T]
            private var size: Int
            
            var count: Int {
                backing.count
            }
            
            var isFullySaturatedUniquely: Bool {
                backing.count == self.size && Set(backing).count == backing.count
            }
            
            init(size: Int) {
                self.size = size
                self.backing = []
            }
            
            mutating func append(_ element: T) {
                if self.backing.count == size {
                    self.backing = Array(self.backing.dropFirst())
                }
                self.backing.append(element)
            }
            
            func contains(_ element: T) -> Bool {
                self.backing.contains(element)
            }
        }

        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> Int {
            try _solution(4)
        }
        
        public func partTwo() throws -> Int {
            try _solution(14)
        }
        
        func _solution(_ size: Int) throws -> Int {
            var characters = Stack<Character>(size: size)

            for (index, char) in try self.puzzleInput.value.enumerated() {
                characters.append(char)
                
                if characters.isFullySaturatedUniquely {
                    return index + 1
                }
            }
            return -1
        }
    }
}
