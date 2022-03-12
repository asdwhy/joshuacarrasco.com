import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ToastrService } from 'ngx-toastr';


@Component({
  selector: 'app-contact',
  templateUrl: './contact.component.html',
  styleUrls: ['./contact.component.scss']
})
export class ContactComponent implements OnInit {

  validateForm: FormGroup;

  constructor(
    private fb: FormBuilder,
    private httpClient: HttpClient,
    private toastr: ToastrService
    ) {
    this.validateForm = this.fb.group({
      username: ['', [Validators.required]],
      email: ['', [Validators.email, Validators.required]],
      message: ['', [Validators.required]]
    });
  }

  ngOnInit(): void {
    
  }

  submitForm(value: { username: string; email: string; message: string }): void {
    for (const key in this.validateForm.controls) {
      if (this.validateForm.controls.hasOwnProperty(key)) {
        this.validateForm.controls[key].markAsDirty();
        this.validateForm.controls[key].updateValueAndValidity();
      }
    }

    // AWS API Gateway to EmailMicroservice
    // This Serverless MS has /default/SendEmail API endpoint which hooks up to an AWS Lambda function
    // Lambda function sends email to appropriate inbox
    this.httpClient.post("https://fdxbjr1bvl.execute-api.us-east-2.amazonaws.com/default/SendEmail", value).subscribe((data:any) => {
      this.toastr.success("Email sent successfully", "Success", {positionClass: 'toast-top-right'});
      this.resetForm();
    });
  }

  resetForm(e?: MouseEvent): void {
    if(e != undefined){
      e.preventDefault();
    }
    this.validateForm.reset();
    for (const key in this.validateForm.controls) {
      if (this.validateForm.controls.hasOwnProperty(key)) {
        this.validateForm.controls[key].markAsPristine();
        this.validateForm.controls[key].updateValueAndValidity();
      }
    }
  }
}
