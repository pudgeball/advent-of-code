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
            
            var isUnique: Bool {
                Set(backing).count == backing.count
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
            var characters = Stack<Character>(size: 4)
            let input = try self.puzzleInput.value
             
            for (index, char) in input.enumerated() {
                characters.append(char)
                
                
                if characters.count == 4 && characters.isUnique {
                    return index + 1
                }
            }
            
            return -1
        }
        
        public func partTwo() throws -> Int {
            var characters = Stack<Character>(size: 14)
            let input = try self.puzzleInput.value
             
            for (index, char) in input.enumerated() {
                characters.append(char)
                
                if characters.count == 14 && characters.isUnique {
                    return index + 1
                }
            }
            return -1
        }
    }
}
