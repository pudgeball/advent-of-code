//
//  PuzzleInput.swift
//  
//
//  Created by Nick McGuire on 2022-12-03.
//

import Foundation

public struct PuzzleInput {
    private enum Backing {
        /// Useful for tests
        case string(String)
        /// Useful for AoC puzzles that are on disk
        case path(URL)
    }
    
    private let backing: Backing
    
    var value: String {
        get throws {
            switch self.backing {
            case .path(let url):
                return try Input.get(filePath: url)
            case .string(let value):
                return value
            }
        }
    }
    
    internal init(_ sourcePath: String = #file, puzzleInputPath: String = "Input.txt") {
        let inputPath = URL(fileURLWithPath: sourcePath).deletingLastPathComponent().appending(path: puzzleInputPath)
        self.backing = .path(inputPath)
    }
    
    internal init(source: String) {
        self.backing = .string(source)
    }
}
