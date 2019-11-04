enum xCoord {
    A = 'A',
    B = 'B',
    C = 'C',
    D = 'D',
    E = 'E',
    F = 'F',
    G = 'G',
}

type yCoord =
    '1' |
    '2' |
    '3' |
    '4' |
    '5' |
    '6' |
    '7';
// type playerType = 'user' | 'computer';

enum Player {
    playerOne,
    playerTwo,
}

// type Owner = Player | undefined;

interface Coord {
    x: xCoord | string;
    y: yCoord | string | number;
}

class Position {
    public readonly position: Coord;
    private _owner?: Player;

    constructor(position: Coord, player?: Player) {
        this.position = position;
        this._owner = player;
    }

    public hasOwner(): boolean {
        return this._owner !== undefined;
    }

    public setOwner(player: Player | undefined) {
        this._owner = player;
    }

    public getOwner(): Player | undefined {
        return this._owner;
    }

    public toString(): string {
        return this.position.x + this.position.y;
    }
}

interface BoardPosition {
    [index: string]: Position;
}

class Board {
    private board: BoardPosition;
    public readonly validPositions: Object;

    constructor(positions?: Position[]) {
        this.board = {};
        this.validPositions = {
            A: [1, 4, 7],
            B: [2, 4, 6],
            C: [3, 4, 5],
            D: [1, 2, 3, 5, 6, 7],
            E: [3, 4, 5],
            F: [2, 4, 6],
            G: [1, 4, 7]
        }
        this.emptyBoard();
        if (!positions) return;
        for (let i = 0; i < positions.length; i++) {
            let coord: string = positions[i].toString();
            this.board[coord] = positions[i];
        }
    }

    public emptyBoard(): void {
        Object.entries(this.validPositions).forEach(([x, arr]) => {
            arr.forEach((y: number) => {
                let coord: string = x+y;
                if (this.board[coord] === undefined) {
                    this.board[coord] = new Position({x: x, y: y});
                }
            })
        });
    }

    public at(position: string): Position {
        return this.board[position];
    }
}