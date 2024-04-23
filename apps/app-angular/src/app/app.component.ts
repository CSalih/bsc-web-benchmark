import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  template: `
    <main>
        <h1>Hello World</h1>
    </main>
  `,
  styles: [],
})
export class AppComponent { }
