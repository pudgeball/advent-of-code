//
//  DaySeven.swift
//  
//
//  Created by Nick McGuire on 2022-12-06.
//

import Foundation

enum Entry {
    case file(String, Int)
    case directory(String)
}

final class Node<T> {
    var value: T
    weak var parent: Node?
    var children: [Node] = []
    
    init(_ value: T) {
        self.value = value
    }
    
    func add(child: Node) {
        self.children.append(child)
        child.parent = self
    }
}

extension Node<Entry> {
    var directorySize: Int {
        switch self.value {
        case .file(_, let size):
            return size
        case .directory(_):
            return children.map(\.directorySize).reduce(0, +)
        }
    }
}

extension AoC2022 {
    public final class DaySeven: Day {
        // MARK: - Structures
        
        
        

        
        // MARK: - Day
        
        let puzzleInput: PuzzleInput
        
        init(_ puzzleInput: PuzzleInput = .init()) {
            self.puzzleInput = puzzleInput
        }
        
        public func partOne() throws -> Int {
            print("This is part one")
            
            var current: Node<Entry>?
            var root: Node<Entry>?
            
            var listingFiles = false
            
            
            for line in try puzzleInput.value.split(separator: "\n") {
                
                
                
                
                
                if let range = line.range(of: "$ cd ") {
                    listingFiles = false
                    
                    print("Current node: \(current) && \(current?.value)")
                    
                    let directoryString = String(line[range.upperBound...])
                    
                    print("Moving to \(directoryString)")
                    
                    
                    if directoryString == ".." {
                        current = current?.parent
                    } else {
                        let directory = Node(Entry.directory(directoryString))
                        
                        if let currentDir = current {
                            currentDir.add(child: directory)
                            current = directory
                        } else {
                            current = directory
                        }
                    }
                    
                    if directoryString == "/" {
                        root = current
                    }
                    
                    
                    
                    continue
                }
                
                if let _ = line.range(of: "$ ls") {
                    listingFiles = true
                    continue
                }
                
                if let _ = line.range(of: "dir "), listingFiles == true {
                    continue
                }
                
                let parts = line.split(separator: " ", maxSplits: 1)
                let size = Int(parts[0])
                let name = String(parts[1])
                
                current?.add(child: .init(.file(name, size!)))
            }
            
            
            print(root?.children.count)
            
            print(current?.children.count)
            
            while current?.parent != nil {
                current = current?.parent
            }
            
            func findDirectory(node: Node<Entry>, maxSize: Int) -> [Node<Entry>] {
                print(node.value, " with size", node.directorySize)
                
                var nodes: [Node<Entry>] = []
                
                switch node.value {
                case .directory(let _):
                    if node.directorySize <= maxSize {
                        nodes.append(node)
                    }
                default:
                    //
                    break
                }
                
                
                
                
                
                for child in node.children {
                    nodes.append(contentsOf: findDirectory(node: child, maxSize: maxSize))
                }
                
                return nodes
            }
            
            
            if let current {
                let matchingDirectorys = findDirectory(node: current, maxSize: 100_000)
                return matchingDirectorys.map(\.directorySize).reduce(0, +)
            }
            return -1
        }
        
        public func partTwo() throws -> Int {
            var current: Node<Entry>?
            var root: Node<Entry>?
            
            var listingFiles = false
            
            
            for line in try puzzleInput.value.split(separator: "\n") {
                if let range = line.range(of: "$ cd ") {
                    listingFiles = false
                    
                    print("Current node: \(current) && \(current?.value)")
                    
                    let directoryString = String(line[range.upperBound...])
                    
                    print("Moving to \(directoryString)")
                    
                    
                    if directoryString == ".." {
                        current = current?.parent
                    } else {
                        let directory = Node(Entry.directory(directoryString))
                        
                        if let currentDir = current {
                            currentDir.add(child: directory)
                            current = directory
                        } else {
                            current = directory
                        }
                    }
                    
                    if directoryString == "/" {
                        root = current
                    }
                    
                    
                    
                    continue
                }
                
                if let _ = line.range(of: "$ ls") {
                    listingFiles = true
                    continue
                }
                
                if let _ = line.range(of: "dir "), listingFiles == true {
                    continue
                }
                
                let parts = line.split(separator: " ", maxSplits: 1)
                let size = Int(parts[0])
                let name = String(parts[1])
                
                current?.add(child: .init(.file(name, size!)))
            }
            
            
            print(root?.children.count)
            
            print(current?.children.count)
            
            while current?.parent != nil {
                current = current?.parent
            }
            
            func findDirectory(node: Node<Entry>, minSize: Int) -> [Node<Entry>] {
//                print(node.value, " with size", node.directorySize)
                
                var nodes: [Node<Entry>] = []
                
                switch node.value {
                case .directory(let _):
                    if node.directorySize >= minSize {
                        nodes.append(node)
                    }
                default:
                    //
                    break
                }
                
                
                
                
                
                for child in node.children {
                    nodes.append(contentsOf: findDirectory(node: child, minSize: minSize))
                }
                
                return nodes
            }
            
            
            if let current {
                let totalSpace = 70_000_000
                let neededSpace = 30_000_000
                
//                24_282_737
                
                let currentSpace = totalSpace - current.directorySize
                
                print(currentSpace)
                
                
                let matchingDirectorys = findDirectory(node: current, minSize: neededSpace - currentSpace)
                return matchingDirectorys.sorted { a, b in
                    return a.directorySize < b.directorySize
                }.first?.directorySize ?? -1
            }
            return -1
        }
    }
}
