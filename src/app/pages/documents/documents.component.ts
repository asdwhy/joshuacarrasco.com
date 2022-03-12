import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-documents',
  templateUrl: './documents.component.html',
  styleUrls: ['./documents.component.scss']
})
export class DocumentsComponent implements OnInit {

  constructor(
    private routerService: Router
  ) { }

  ngOnInit(): void {
  }

  openResume(): void {
    window.open("assets/resume-updated.pdf", "_blank");
  }

  openCoverletter(): void {
    window.open("assets/CoverLetter_software_dev@caseware.pdf", "_blank");
  }

}
