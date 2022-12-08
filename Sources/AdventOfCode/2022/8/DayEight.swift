//
//  DayEight.swift
//  
//
//  Created by Nick McGuire on 2022-12-07.
//

import Foundation

extension AoC2022 {
    public final class DayEight: Day {
        // MARK: - Structures
        

        
        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> Int {
            let trees: [[Int]] = try puzzleInput.value.split(separator: "\n").map { line in
                return line.split(separator: "").compactMap { char in
                    return Int(char)
                }
            }

            var visibleTreeCount = (trees.count * 2 + trees[0].count * 2) - 4

            for y in 1..<(trees.count - 1) {
                for x in 1..<(trees[y].count - 1) {
                    let tree = trees[y][x]

                    let northVisible = {
                        for n in stride(from: y - 1, through: 0, by: -1) {
                            if trees[n][x] >= tree {
                                return false
                            }
                        }
                        return true
                    }
                    let southVisible = {
                        for s in stride(from: y + 1, to: trees.count, by: 1) {
                            if trees[s][x] >= tree {
                                return false
                            }
                        }
                        return true
                    }
                    let eastVisible = {
                        for e in stride(from: x + 1, to: trees.count, by: 1) {
                            if trees[y][e] >= tree {
                                return false
                            }
                        }
                        return true
                    }
                    let westVisible = {
                        for w in stride(from: x - 1, through: 0, by: -1) {
                            if trees[y][w] >= tree {
                                return false
                            }
                        }
                        return true
                    }
                    
                    if northVisible() || southVisible() || eastVisible() || westVisible() {
                        visibleTreeCount += 1
                    }
                }
            }
            return visibleTreeCount
        }
        
        public func partTwo() throws -> Int {
            let trees: [[Int]] = try puzzleInput.value.split(separator: "\n").map { line in
                return line.split(separator: "").compactMap { char in
                    return Int(char)
                }
            }

            var mostScenicScore = 0
            
            for y in 1..<(trees.count - 1) {
                for x in 1..<(trees[y].count - 1) {
                    let tree = trees[y][x]

                    let northScore = {
                        var sc = 0
                        for n in stride(from: y - 1, through: 0, by: -1) {
                            sc += 1
                            if trees[n][x] >= tree {
                                break
                            }
                        }
                        return sc
                    }
                    let southScore = {
                        var sc = 0
                        for s in stride(from: y + 1, to: trees.count, by: 1) {
                            sc += 1
                            if trees[s][x] >= tree {
                                break
                            }
                        }
                        return sc
                    }
                    let eastScore = {
                        var sc = 0
                        for e in stride(from: x + 1, to: trees.count, by: 1) {
                            sc += 1
                            if trees[y][e] >= tree {
                                break
                            }
                        }
                        return sc
                    }
                    let westScore = {
                        var sc = 0
                        for w in stride(from: x - 1, through: 0, by: -1) {
                            sc += 1
                            if trees[y][w] >= tree {
                                break
                            }
                        }
                        return sc
                    }

                    let score = northScore() * southScore() * eastScore() * westScore()
                    if score > mostScenicScore {
                        mostScenicScore = score
                    }
                }
            }
            return mostScenicScore
        }
    }
}
