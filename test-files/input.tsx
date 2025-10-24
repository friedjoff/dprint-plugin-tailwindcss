import React from 'react';

// Test 1: Simple component with unsorted classes
export function Button() {
    return (
        <button className="z-10 hover:shadow-lg p-4 bg-blue-500 text-white rounded-lg">
            Click me
        </button>
    );
}

// Test 2: Component with clsx
import clsx from 'clsx';

export function Card({ variant }) {
    return (
        <div className={clsx(
            "shadow-lg rounded-lg p-6 bg-white",
            "hover:shadow-xl transition-shadow",
            variant === 'primary' && "border-blue-500 border-2"
        )}>
            Card content
        </div>
    );
}

// Test 3: Complex component
export function Header() {
    return (
        <header className="flex items-center justify-between w-full h-16 px-4 bg-gray-800 text-white">
            <h1 className="text-2xl font-bold">Logo</h1>
            <nav className="space-x-4 flex">
                <a href="#" className="hover:text-blue-400 text-gray-300">Home</a>
                <a href="#" className="hover:text-blue-400 text-gray-300">About</a>
            </nav>
        </header>
    );
}

// Test 4: With important and negative values
export function Alert() {
    return (
        <div className="!bg-red-500 -mt-4 p-4 text-white rounded-lg shadow-lg">
            Alert message
        </div>
    );
}

// Test 5: Responsive design
export function ResponsiveBox() {
    return (
        <div className="xl:text-xl lg:text-lg md:text-base text-sm p-4 bg-white">
            Responsive text
        </div>
    );
}
