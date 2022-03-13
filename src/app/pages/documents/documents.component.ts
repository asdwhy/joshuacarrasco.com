import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-documents',
  templateUrl: './documents.component.html',
  styleUrls: ['./documents.component.scss']
})
export class DocumentsComponent implements OnInit {

  ngOnInit(): void {
  }

  openResume(): void {
    window.open("assets/resume.pdf", "_blank");
  }

  openCoverletter(): void {
    window.open("assets/coverletter.pdf", "_blank");
  }

}
