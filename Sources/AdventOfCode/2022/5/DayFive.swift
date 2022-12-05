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
            
            init(_ input: String) {
                var stacks: [[Character]] = []
                
                let regex = try! NSRegularExpression(pattern: "move (\\d) from (\\d) to (\\d)")
                
                let parts = input.split(separator: "\n\n")
                let map = parts[0]
                let instructions = parts[1]
                
//                print("map:\n\(map)")
//                print("instructions: \(instructions)")
                
                // Initialize the storage
                for _ in 0..<String(map.split(separator: "\n").last!).chunks(ofCount: 4).count {
                    stacks.append([])
                }
                
                print("====")
                for line in map.split(separator: "\n").dropLast().reversed() {
                    for (i, stack) in String(line).chunks(ofCount: 4).enumerated() {
                        let char = String(stack).dropFirst().first!
                        if !char.isWhitespace {
                            stacks[i].append(char)
                        }
                    }
                }
                print("====")
                
                print("{\(stacks.enumerated().map { (i, s) in return "'\(i+1)': '\(String(s))'" }.joined(separator: ", "))}")
                
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
                        
                        for _ in 0..<moveCount {
                            let c = stacks[initialStack - 1].last!
                            stacks[initialStack - 1] = Array(stacks[initialStack - 1].dropLast())
                            stacks[finalStack - 1].append(c)
                        }
                        
                        print("{\(stacks.enumerated().map { (i, s) in return "'\(i+1)': '\(String(s))'" }.joined(separator: ", "))}")
                    }
                }
                
                print(stacks)
                print(stacks.map({ String($0) }))
                
                self.stacks = stacks
                
                //DVWBGHWNP
                //WZCRHV
            }
            
            func result() -> String {
                stacks.compactMap(\.last).map(String.init).reduce("", +)
            }
        }
        
        
        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> String {
            CrateMap(try puzzleInput.value).result()
        }
        
        public func partTwo() throws -> String {
            ""
        }
    }
}
