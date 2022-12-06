//
//  File.swift
//  
//
//  Created by Nick McGuire on 2022-12-04.
//

import Foundation
import Algorithms

extension AoC2022 {
    public final class DayFive: Day {
        // MARK: - Structures
        
        internal struct CrateMap {
            private let stacks: [[Character]]
            
            init(_ input: String, moveTogether: Bool) {
                var stacks: [[Character]] = []
                
                let regex = try! NSRegularExpression(pattern: #"move (\d+) from (\d+) to (\d+)"#)
                
                let parts = input.split(separator: "\n\n")
                let map = parts[0]
                let instructions = parts[1]
                
//                print("map:\n\(map)")
//                print("instructions: \(instructions)")
                
                // Initialize the storage
                for _ in 0..<String(map.split(separator: "\n").last!).chunks(ofCount: 4).count {
                    stacks.append([])
                }
                
//                print("====")
                for line in map.split(separator: "\n").dropLast() {
                    for (i, stack) in String(line).chunks(ofCount: 4).enumerated() {
                        let char = String(stack).dropFirst().first!
                        if !char.isWhitespace {
                            stacks[i].append(char)
                        }
                    }
                }
//                print("====")
                
//                print(stacks.map({ Array($0.reversed()) }))
                
//                print("{\(stacks.enumerated().map { (i, s) in return "'\(i+1)': '\(String(s))'" }.joined(separator: ", "))}")
//                print("{\(stacks.enumerated().map { (i, s) in return "'\(i+1)': \(s.count)" }.joined(separator: ", "))}")
                
                let nsrange = NSRange(instructions.startIndex..<instructions.endIndex,
                                      in: instructions)
                regex.enumerateMatches(in: String(instructions),
                                       options: [],
                                       range: nsrange) { (match, _, stop) in
                    guard let match = match else { return }

                    if match.numberOfRanges == 4,
                       let firstCaptureRange = Range(match.range(at: 1),
                                                     in: instructions),
                       let secondCaptureRange = Range(match.range(at: 2),
                                                      in: instructions),
                       let thirdCaptureRange = Range(match.range(at: 3),
                                                      in: instructions),
                       let moveCount = Int(instructions[firstCaptureRange]),
                       let initialStack = Int(instructions[secondCaptureRange]),
                       let finalStack = Int(instructions[thirdCaptureRange])
                    {
//                        print("move \(moveCount) from \(initialStack) to \(finalStack)")
                        
                        
                        var moveStack: [Character] = []
                        for _ in 0..<moveCount {
                            if moveTogether {
                                moveStack.append(stacks[initialStack - 1].removeFirst())
                            } else {
                                let c = stacks[initialStack - 1].removeFirst()
                                stacks[finalStack - 1].insert(c, at: 0)
                            }
                        }
                        
                        for c in moveStack.reversed() {
                            stacks[finalStack - 1].insert(c, at: 0)
                        }
                    }
                }
                
//                print(stacks)
//                print(stacks.map({ String($0) }))
//                print(stacks.map(\.count))
                
                self.stacks = stacks
                
                //DVWBGHWNP
                //WZCRHV
            }
            
            func result() -> String {
                stacks.compactMap(\.first).map(String.init).reduce("", +)
            }
        }
        
        
        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> String {
            CrateMap(try puzzleInput.value, moveTogether: false).result()
        }
        
        public func partTwo() throws -> String {
            CrateMap(try puzzleInput.value, moveTogether: true).result()
        }
    }
}
