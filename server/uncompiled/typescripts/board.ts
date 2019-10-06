enum xCoord {
    A = 'A',
    B = 'B',
    C = 'C',
    D = 'D',
    E = 'E',
    F = 'F',
    G = 'G',
}

enum yCoord {
    one = '1',
    two = '2',
    three = '3',
    four = '4',
    five = '5',
    six = '6',
    seven = '7',
}

// type playerType = 'user' | 'computer';

enum Player {
    playerOne,
    playerTwo,
}

// type Owner = Player | undefined;

interface Coord {
    x: xCoord;
    y: yCoord;
}

class Position {
    public readonly position: Coord;
    private _owner?: Player;

    constructor(position: Coord, player?: Player) {
        this.position = position;
        this._owner = player;
    }

    public hasOwner(player?: Player): boolean {
        return this._owner === player;
    }

    public set owner(player: Player | undefined) {
        this._owner = player;
    }

    public get owner(): Player | undefined {
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

    constructor(positions?: Position[]) {
        this.board = {};
        this.emptyBoard();
        if (!positions) return;
        for (let i = 0; i < positions.length; i++) {
            let coord: string = positions[i].toString();
            this.board[coord] = positions[i];
        }
    }

    public emptyBoard(): void {
        Object.values(xCoord).forEach(x => {
            Object.values(yCoord).forEach(y => {
                let coord: string = x+y;
                if (this.board[coord] === undefined) {
                    this.board[coord] = new Position({x: x, y: y});
                }
            })
        });
    }

    public at(position: Coord | Position): Position {
        let coord: Coord = position instanceof Position ? position.position : position;
        return this.board[coord.x + coord.y];
    }
}