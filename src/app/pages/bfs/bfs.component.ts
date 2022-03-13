import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-bfs',
  templateUrl: './bfs.component.html',
  styleUrls: ['./bfs.component.scss']
})
export class BfsComponent implements OnInit {

  constructor(
  ) { }

  ngOnInit(): void {
  }

  openDemo(): void {
    window.open("https://learnera.joshuacarrasco.com", "_blank");
  }
}
