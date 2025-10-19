class Graph {
	constructor(thresholds, min, max) {
		// min is currently ignored, supposed 0
		this.thresholds = thresholds
		this.minValue = min
		this.maxValue = max
		this.fill_char = 'o'
		this.empty_char = '.'
		this.values = []
		this.MAX_VALUE_COUNT = 100
		this.width = 30
		this.height = 10
		this.columns = []
	}

	drawWithNewValue(value) {
		this.values.push(value)
		while (this.values.length > this.MAX_VALUE_COUNT) this.values.shift()

		const computeHeight = (value) => { return Math.round((this.height - 1) * value / this.maxValue) }

		let fillerHeight = computeHeight(value)
		let thresholdHeights = this.thresholds.map(threshold => computeHeight(threshold))
		let column = []
		for (let r = 0; r < this.height; r++) {
			let char = r == fillerHeight ? this.fill_char : this.empty_char
			let color = (r <= fillerHeight && r > thresholdHeights[1]) ? 'r' :
				(r <= fillerHeight && r > thresholdHeights[0]) ? 'y' :
					(r <= fillerHeight) ? 'g' : 'bg'
			column.push([char, color])
		}
		this.columns.push(column)
		while (this.columns.length > this.width) this.columns.shift()

		let graph = ""
		for (let r = 0; r < this.height; r++) {
			for (let c = 0; c < this.width; c++) {
				if (this.columns[c]) {
					let char = this.columns[c][this.height - 1 - r][0]
					let color = this.columns[c][this.height - 1 - r][1]
					graph += `<span class='${color}'>${char}</span>`
				} else {
					graph += `<span class='bg'>${this.empty_char}</span>`
				}
			}
			graph += "\n"
		}
		/** columns
		 * 	[
		 * 		[ [o g], [o g], [o g], [. y], [. y], [. y], [. r], [. r] ],
		 * 		[ [o g], [o g], [o g], [. y], [. y], [. y], [. r], [. r] ],
		 * 		[ [o g], [o g], [o g], [o y], [. y], [. y], [. r], [. r] ],
		 * 		[ [o g], [o g], [o g], [o y], [o y], [. y], [***], [. r] ]
		 * 	]
		 *
		 * graph:
		 * 	....
		 * 	...*
		 * 	....
		 * 	...o
		 * 	..oo
		 * 	oooo
		 * 	oooo
		 * 	oooo
		 *
		 * graph[1][3] = columns[3][6], so columns[col][height-1-row]
		 */
		return graph
	}
}


const Color = {
	Red: 'red',
	Yellow: 'yellow',
	Green: 'green',
	Grey: 'grey'
}

class Tile {
	constructor(char, color) {
		this.char = char
		this.color = color
	}

	toHtml() { return `<span style="color: ${this.color};">${this.char}</span>` }
}

class Canvas {
	constructor(width, height, bg_tile) {
		this.width = width
		this.height = height
		this.bg_tile = bg_tile
		this.table = []
		/** 
		 * 	table: [
		 * 		[ r0, r1, r2, ... ], 	// column 0
		 * 		[ r0, r1, r2, ... ],	// column 1
		 * 		[ r0, r1, r2, ... ],	// column 2
		 * 		...
		 * 	]
		 *  
		 */
	}

	fillColumn(col, tile) {
		let newColumn = []
		for (let r=0; r < this.height; r++) newColumn.push(tile)
		this.table[col] = newColumn
	}

	empty() {
		for (let c=0; c < this.width; c++) this.fillColumn(c, this.bg_tile)
	}

	addAndShiftColumn(column) {
		// let newColumn = []
		// for (let r=0; r < this.height; r++) newColumn.push(tile)
		if (column.length != this.height) console.log("MALEMALE")
		this.table.push(column)
		while (this.table.length > this.width) this.table.shift()
	}

	shiftColumns() {
		this.table.shift()
		this.fillColumn(this.table.length-1, this.bg_tile)
  	}

	toHtml() {
		let htmlString = ""
		for (let r=0; r < this.height; r++) {
			for (let c=0; c < this.width; c++) {
				htmlString += this.table[c][this.height-1-r].toHtml()		
			}
			htmlString += '\n'
		}
		/**
		 * table
		 * 	[ r0, r1, r2, r3, r4, r5, r6, r7 ],
		 * 	[ r0, r1, r2, r3, r4, r5, r6, r7 ],
		 * 	[ r0, r1, r2, r3, r4, r5, r6, r7 ],
		 * 	[ r0, r1, r2, r3, r4, r5, **, r7 ],
		 * 
		 * htmlString
		 * 	....\n
		 * 	...*\n
		 * 	....\n
		 * 	...o\n
		 * 	..oo\n
		 * 	oooo\n
		 * 	oooo\n
		 * 	oooo\n
		 * 
		 * htmlString[9] = table[3][6]
		 * htmlStirng[i] = table[col][height-1-row]
		 */
		return htmlString
	}

}

class Graphic {
	constructor(min, max, thresholds, width, height, hasLegend) {
		this.min = min
		this.max = max
		this.thresholds = thresholds
		this.canvas = new Canvas(width, height, new Tile('.', Color.Grey))
		this.canvas.empty()
		this.values = []
		this.newValuesCount = 0
		this.MAX_VALUE_COUNT = 100
		
		if (hasLegend == true) {}
	}

	normalize(value, min, max) {
		return (value-min) / (max-min)
	}

	computeThresholds(thresholds) {
		const minColorV = 90
		const maxColorV = 0
		let coloredThresholds = thresholds.map((threshold) => {
			let normThreshold = this.normalize(threshold, this.min, this.max)
			let colorV = minColorV - normThreshold * Math.abs(maxColorV - minColorV)

		})

		return thresholds
	}

	addValue(value) {
		this.values.push(value)
		while (this.values.length > this.MAX_VALUE_COUNT) this.values.shift()
		
		this.newValuesCount++
	}

	updateGraph() {
		let newValues = this.values.slice(-this.newValuesCount)

		let newNormalizedValues = newValues.map((value) => this.normalize(value, this.min, this.max))
		let normThresholds = this.thresholds.map((threshold) => this.normalize(threshold, this.min, this.max))
		/**
		 * Example: 
		 * 	min: 20, max: 90, thresholds: [55, 80]
		 * 	newValues: [33, 37, 51, 69, 84]
		 * 
		 * 	newNormalizedValues: [0.19, 0.24, 0.44, 0.70, 0.91]
		 * 	normThresholdsRev: [0.50, 0.86]
		 */
		for (let normValue of newNormalizedValues) {
			let column = this.buildColumn(normValue, normThresholds)
			this.canvas.addAndShiftColumn(column)
		}

		this.newValuesCount = 0
	}

	buildColumn(normValue, normThresholds) {
		const empty_char = '.'
		const filled_char = 'o'
		const minColorHue = 110
		const maxColorHue = 0
		
		const biggerNormThresholdPassed = normThresholds.findLast((normThreshold) => normThreshold < normValue)
		const valueHeight = Math.round(normValue * (this.canvas.height-1))

		const column = []

		for (let r=0; r < this.canvas.height; r++) {
			const color = (() => {
				if (r > valueHeight) {
					return 'grey'
				} else {
					// const colorHue = minColorHue - (biggerNormThresholdPassed?biggerNormThresholdPassed:0) * Math.abs(maxColorHue - minColorHue)
					// const colorHue = minColorHue - normValue * Math.abs(maxColorHue - minColorHue)
					const colorHue = minColorHue - (r/this.canvas.height) * Math.abs(maxColorHue - minColorHue)
					console.log(colorHue)
					return `hsl(${colorHue},100%,50%)`
				}				
			})()
			const char = r <= valueHeight ? filled_char : empty_char

			const tile = new Tile(char, color)
			column.push(tile)
		}

		return column
	}

	print() {
		let htmlString = this.canvas.toHtml() + "\n\n"
		return htmlString 
	}


}