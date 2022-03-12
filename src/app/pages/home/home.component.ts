import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  data: String[] = [
    "3rd year at University of Toronto Scarborough",
    "Computer Science co-op specialist - Software Engineering"
  ];

  constructor() { }

  ngOnInit() {
  }

}
